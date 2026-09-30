use crate::{controls::ControlTarget, types::*, world::PerformanceState};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SNAPSHOT_SCHEMA_VERSION: u32 = 1;

/// An owned point-in-time copy, not a view into mutable runtime state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    pub schema_version: u32,
    pub snapshot_id: SnapshotId,
    /// Supplied by the owner; this model does not track mutations or revisions.
    pub state_revision: u64,
    pub captured_at: Timestamp,
    pub performance_time: PerformanceTime,
    pub state: PerformanceState,
    pub observation: Option<ObservationMetadata>,
    pub measurements: Option<SoundMeasurements>,
    pub summaries: Option<DecisionSummaries>,
}

impl PerformanceSnapshot {
    /// The caller must provide a coherent state and its associated time/revision.
    pub fn capture(state: &PerformanceState, identity: SnapshotIdentity) -> Self {
        Self {
            schema_version: SNAPSHOT_SCHEMA_VERSION,
            snapshot_id: identity.id,
            state_revision: identity.state_revision,
            captured_at: identity.captured_at,
            performance_time: identity.performance_time,
            state: state.clone(),
            observation: None,
            measurements: None,
            summaries: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotIdentity {
    pub id: SnapshotId,
    pub state_revision: u64,
    pub captured_at: Timestamp,
    pub performance_time: PerformanceTime,
}

/// UTC time since the Unix epoch, in integer nanoseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timestamp(pub i64);

/// Session-relative monotonic time; session identity prevents cross-session comparisons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceTime {
    pub session_id: String,
    pub elapsed_nanoseconds: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ObservationMetadata {
    pub entities: Vec<EntityObservation>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityObservation {
    pub entity: EntityRef,
    pub connected: StateValue<bool>,
    pub last_observed_at: StateValue<Timestamp>,
    /// Field paths relative to the entity, for partially observed state.
    pub fields: BTreeMap<String, FieldObservation>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldObservation {
    pub observed_at: Timestamp,
    pub source: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SoundMeasurements {
    pub levels: Vec<SignalMeasurement>,
    pub audio_windows: Vec<AudioWindowReference>,
    pub effect_residuals: Vec<EffectResidualMeasurement>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasurementWindow {
    pub start: PerformanceTime,
    pub end: PerformanceTime,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalMeasurement {
    pub entity: EntityRef,
    pub window: MeasurementWindow,
    pub rms_dbfs: StateValue<Decibels>,
    pub peak_dbfs: StateValue<Decibels>,
    pub clipping: StateValue<bool>,
    pub signal_present: StateValue<bool>,
    pub presence_threshold_dbfs: StateValue<Decibels>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioWindowReference {
    pub bus: BusId,
    pub window: MeasurementWindow,
    pub uri: String,
    pub sample_rate_hz: u32,
    pub channels: u16,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectResidualMeasurement {
    pub unit: EffectUnitId,
    pub window: MeasurementWindow,
    pub level_dbfs: StateValue<Decibels>,
    pub estimated_remaining: StateValue<Seconds>,
}

/// Optional, non-authoritative projections. Never used to replace world values.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DecisionSummaries {
    pub decks: BTreeMap<DeckId, DeckSummary>,
    pub timing_differences: Vec<TimingDifferenceSummary>,
    pub routing_paths: Vec<RoutingPathSummary>,
    pub control_meanings: Vec<ControlMeaningSummary>,
}
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DeckSummary {
    pub time_remaining: StateValue<Seconds>,
    pub beats_remaining: StateValue<Beats>,
    pub loop_duration: StateValue<Seconds>,
    pub loop_beats: StateValue<Beats>,
    pub upcoming: Vec<UpcomingBoundary>,
    pub available_cues: Vec<CueId>,
    pub available_loops: Vec<LoopId>,
    pub analysis_section: StateValue<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpcomingBoundary {
    pub label: String,
    pub position: Position,
    pub time_until: StateValue<Seconds>,
    pub beats_until: StateValue<Beats>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimingDifferenceSummary {
    pub reference: DeckId,
    pub compared: DeckId,
    pub bpm_difference: StateValue<Bpm>,
    pub phase_difference: StateValue<Beats>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoutingPathSummary {
    pub source: SignalSource,
    pub channel: StateValue<ChannelId>,
    pub effects: Vec<EffectUnitId>,
    pub buses: Vec<BusId>,
    pub effective_gain: StateValue<Decibels>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlMeaningSummary {
    pub surface: ControlSurfaceId,
    pub control: ControlId,
    pub target: StateValue<ControlTarget>,
    pub action: String,
}
