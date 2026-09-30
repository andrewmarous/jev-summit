use jev_summit::*;
use std::collections::BTreeMap;

fn identity() -> SnapshotIdentity {
    SnapshotIdentity {
        id: "snapshot-1".into(),
        state_revision: 42,
        captured_at: Timestamp(1_800_000_000_000_000_000),
        performance_time: PerformanceTime {
            session_id: "session-1".into(),
            elapsed_nanoseconds: 10_000_000_000,
        },
    }
}

fn world() -> PerformanceState {
    let mut state = PerformanceState::default();
    let track = TrackState {
        asset_version: "v1".into(),
        metadata: TrackMetadata {
            title: StateValue::Known("Example".into()),
            artist: StateValue::Unknown,
            album: StateValue::Unset,
            additional: BTreeMap::new(),
        },
        source: TrackSource {
            kind: SourceKind::AudioFile,
            locator: StateValue::Known("file:///music/example.wav".into()),
            duration: StateValue::Known(Seconds(180.0)),
            sample_rate_hz: StateValue::Known(48_000),
            channels: StateValue::Known(2),
        },
        preparation: TrackPreparation {
            revision: "prep-1".into(),
            analysis_revision: StateValue::Unknown,
            original_key: StateValue::Unknown,
            key_analysis: StateValue::Unsupported,
            beat_grid: StateValue::Known(BeatGrid {
                revision: "grid-1".into(),
                anchors: vec![BeatGridAnchor {
                    position: SourcePosition {
                        frame: 0,
                        sample_rate_hz: 48_000,
                    },
                    beat: Beats(0.0),
                    tempo: Bpm(120.0),
                }],
            }),
            hot_cues: BTreeMap::new(),
            loops: BTreeMap::new(),
            markers: BTreeMap::new(),
            analyzed_gain: StateValue::Unknown,
            unsaved_edits: StateValue::Unset,
            sections: StateValue::Unsupported,
        },
    };
    state.tracks.insert("track-1".into(), track);
    for (deck_id, instance_id, frame) in [
        ("deck-1", "instance-1", 0),
        ("deck-2", "instance-2", 96_000),
    ] {
        let mut deck = DeckState {
            content: DeckContent::Ready {
                track: "track-1".into(),
                instance: instance_id.into(),
            },
            ..DeckState::default()
        };
        deck.transport.position = StateValue::Known(SourcePosition {
            frame,
            sample_rate_hz: 48_000,
        });
        deck.cue.temporary_cue = StateValue::Unset;
        deck.looping.enabled = StateValue::Known(false);
        deck.looping.boundaries = StateValue::Known(LoopBoundaries {
            start: Position::Source(SourcePosition {
                frame: 0,
                sample_rate_hz: 48_000,
            }),
            end: Position::Source(SourcePosition {
                frame: 96_000,
                sample_rate_hz: 48_000,
            }),
        });
        state.decks.insert(deck_id.into(), deck);
    }
    state.mixer.channels.insert(
        "channel-a".into(),
        ChannelState {
            source: StateValue::Known(SignalSource::Deck("deck-2".into())),
            fader: StateValue::Known(NormalizedValue(0.0)),
            ..ChannelState::default()
        },
    );
    state.configuration.capabilities.push(EntityCapabilities {
        entity: EntityRef::Deck("deck-1".into()),
        supported_features: StateValue::Unknown,
        parameter_ranges: BTreeMap::new(),
        supported_modes: BTreeMap::new(),
    });
    state.controls.surfaces.insert(
        "surface-1".into(),
        SurfaceControlState {
            physical_controls: BTreeMap::from([(
                "fader-1".into(),
                PhysicalControlState {
                    position: StateValue::Known(NormalizedValue(0.8)),
                    target: StateValue::Known(ControlTarget {
                        entity: EntityRef::Channel("channel-a".into()),
                        parameter: "fader".into(),
                    }),
                    pickup: StateValue::Known(PickupState::Waiting {
                        effective_position: NormalizedValue(0.0),
                        signed_mismatch: 0.8,
                    }),
                },
            )]),
            ..SurfaceControlState::default()
        },
    );
    state
}

#[test]
fn snapshot_round_trips_through_json() {
    let snapshot = PerformanceSnapshot::capture(&world(), identity());
    let json = serde_json::to_string_pretty(&snapshot).unwrap();
    let decoded: PerformanceSnapshot = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, snapshot);
    assert!(json.contains("\"deck-1\""));
    assert_eq!(snapshot.schema_version, SNAPSHOT_SCHEMA_VERSION);
}

#[test]
fn snapshot_is_independent_of_live_world() {
    let mut runtime = PerformanceRuntime { state: world() };
    let snapshot = PerformanceSnapshot::capture(&runtime.state, identity());
    runtime
        .state
        .decks
        .get_mut(&DeckId::from("deck-1"))
        .unwrap()
        .content = DeckContent::Unloaded;
    runtime.state.tracks.clear();
    assert!(matches!(
        snapshot.state.decks[&DeckId::from("deck-1")].content,
        DeckContent::Ready { .. }
    ));
    assert_eq!(snapshot.state.tracks.len(), 1);
}

#[test]
fn absence_states_and_zero_are_distinct() {
    let values: Vec<StateValue<u64>> = vec![
        StateValue::Known(0),
        StateValue::Unset,
        StateValue::Unknown,
        StateValue::Unsupported,
    ];
    let encoded: Vec<_> = values
        .iter()
        .map(|v| serde_json::to_string(v).unwrap())
        .collect();
    for (i, json) in encoded.iter().enumerate() {
        assert_eq!(
            serde_json::from_str::<StateValue<u64>>(json).unwrap(),
            values[i]
        );
        for other in encoded.iter().skip(i + 1) {
            assert_ne!(json, other);
        }
    }
}

#[test]
fn decks_share_preparation_but_not_execution() {
    let state = world();
    let a = &state.decks[&DeckId::from("deck-1")];
    let b = &state.decks[&DeckId::from("deck-2")];
    match (&a.content, &b.content) {
        (
            DeckContent::Ready {
                track: ta,
                instance: ia,
            },
            DeckContent::Ready {
                track: tb,
                instance: ib,
            },
        ) => {
            assert_eq!(ta, tb);
            assert_ne!(ia, ib);
        }
        _ => panic!("expected loaded decks"),
    }
    assert_ne!(a.transport.position, b.transport.position);
    assert!(matches!(a.looping.boundaries, StateValue::Known(_)));
    assert_eq!(a.looping.enabled, StateValue::Known(false));
}

#[test]
fn tagged_variants_round_trip() {
    let values = vec![
        SlipState::Disabled,
        SlipState::Armed,
        SlipState::Running {
            underlying_position: SourcePosition {
                frame: 100,
                sample_rate_hz: 48_000,
            },
            retained_rate: PlaybackRate(1.0),
            direction: PlaybackDirection::Forward,
            return_behavior: ReturnBehavior::ResumeUnderlying,
        },
    ];
    for value in values {
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(serde_json::from_str::<SlipState>(&json).unwrap(), value);
    }
    let processing = BusProcessing::Other {
        name: "recording".into(),
        parameters: BTreeMap::new(),
    };
    let json = serde_json::to_string(&processing).unwrap();
    assert_eq!(
        serde_json::from_str::<BusProcessing>(&json).unwrap(),
        processing
    );
}
