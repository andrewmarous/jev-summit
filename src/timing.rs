use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DeckTimingState {
    pub grid: StateValue<BeatGridRef>,
    pub position: StateValue<MusicalPosition>,
    pub beat_index: StateValue<i64>,
    /// Fraction of a beat in [0, 1).
    pub phase: StateValue<f64>,
    pub previous_beat: StateValue<SourcePosition>,
    pub next_beat: StateValue<SourcePosition>,
    pub closest_beat: StateValue<SourcePosition>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SynchronizationState {
    pub decks: BTreeMap<DeckId, DeckSyncState>,
    pub leader: StateValue<SyncLeader>,
    pub shared_clock: StateValue<SharedClock>,
    pub relationships: Vec<DeckTimingRelationship>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeckSyncState {
    pub member: StateValue<bool>,
    pub mode: StateValue<SyncMode>,
    pub retained: StateValue<RetainedSyncState>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum SyncMode {
    Off,
    TempoOnly,
    TempoAndPhase,
    Other(String),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "deck", rename_all = "snake_case")]
pub enum SyncLeader {
    Deck(DeckId),
    SharedClock,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedClock {
    pub tempo: Bpm,
    pub beat: Beats,
    pub phase: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetainedSyncState {
    pub target_tempo: StateValue<Bpm>,
    pub phase_offset: StateValue<Beats>,
    pub rate_before_sync: StateValue<PlaybackRate>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeckTimingRelationship {
    pub reference: DeckId,
    pub compared: DeckId,
    /// Compared minus reference.
    pub tempo_difference: StateValue<Bpm>,
    pub phase_difference: StateValue<Beats>,
}
