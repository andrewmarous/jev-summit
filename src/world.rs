use crate::{
    controls::ControlState, deck::DeckState, effects::EffectUnitState, mixer::MixerState,
    timing::SynchronizationState, track::TrackState, types::*,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Authoritative domain state, owned persistently by the runtime.
/// Map keys are entity identities; IDs are not duplicated inside their values.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PerformanceState {
    pub configuration: PerformanceConfiguration,
    pub tracks: BTreeMap<TrackId, TrackState>,
    pub decks: BTreeMap<DeckId, DeckState>,
    pub mixer: MixerState,
    pub effects: BTreeMap<EffectUnitId, EffectUnitState>,
    pub synchronization: SynchronizationState,
    pub controls: ControlState,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PerformanceConfiguration {
    pub output_sample_rate_hz: StateValue<u32>,
    pub output_latency: StateValue<Seconds>,
    pub playback_options: BTreeMap<String, ParameterState>,
    /// A vector avoids JSON object keys containing structured EntityRef values.
    pub capabilities: Vec<EntityCapabilities>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityCapabilities {
    pub entity: EntityRef,
    pub supported_features: StateValue<BTreeSet<String>>,
    pub parameter_ranges: BTreeMap<ParameterId, ParameterRange>,
    pub supported_modes: BTreeMap<String, Vec<String>>,
}

/// Ownership boundary only; connections, DSP handles and update logic are excluded.
#[derive(Debug, Default)]
pub struct PerformanceRuntime {
    pub state: PerformanceState,
}
