use crate::{deck::ReturnBehavior, types::*};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ControlState {
    pub software: SoftwareControlState,
    pub surfaces: BTreeMap<ControlSurfaceId, SurfaceControlState>,
}
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SoftwareControlState {
    pub operating_mode: StateValue<OperatingMode>,
    pub browser: StateValue<BrowserState>,
    pub selected_deck: StateValue<DeckId>,
    pub selected_effect_unit: StateValue<EffectUnitId>,
    pub selected_effect_target: StateValue<EffectTarget>,
    pub selected_parameter_layer: StateValue<String>,
    pub selected_prepared_entry: StateValue<PreparedEntryRef>,
    pub screen_mode: StateValue<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BrowserState {
    pub selected_track: StateValue<TrackId>,
    pub selected_collection: StateValue<String>,
    pub load_target: StateValue<DeckId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PreparedEntryRef {
    Cue { track: TrackId, cue: CueId },
    Loop { track: TrackId, looping: LoopId },
    Marker { track: TrackId, marker: MarkerId },
}
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SurfaceControlState {
    pub sections: BTreeMap<String, DeckSectionState>,
    pub active_modifiers: BTreeSet<String>,
    pub held_buttons: BTreeMap<ControlId, HeldButtonState>,
    pub selected_effect_unit: StateValue<EffectUnitId>,
    pub selected_effect_target: StateValue<EffectTarget>,
    pub parameter_layer: StateValue<String>,
    pub screen_mode: StateValue<String>,
    pub physical_controls: BTreeMap<ControlId, PhysicalControlState>,
}
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DeckSectionState {
    pub deck: StateValue<DeckId>,
    pub pad_mode: StateValue<PadMode>,
    pub pad_bank: StateValue<u32>,
    pub jog_touched: StateValue<bool>,
    pub jog_mode: StateValue<JogMode>,
    pub jog_feel: StateValue<NormalizedValue>,
    pub vinyl_start_time: StateValue<Seconds>,
    pub vinyl_stop_time: StateValue<Seconds>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum PadMode {
    HotCue,
    BeatLoop,
    BeatJump,
    LoopRoll,
    Sampler,
    Other(String),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum JogMode {
    Vinyl,
    Nudge,
    Search,
    Other(String),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeldButtonState {
    pub semantic_action: String,
    pub target: StateValue<EntityRef>,
    pub release_behavior: StateValue<ReturnBehavior>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalControlState {
    pub position: StateValue<NormalizedValue>,
    pub target: StateValue<ControlTarget>,
    pub pickup: StateValue<PickupState>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlTarget {
    pub entity: EntityRef,
    pub parameter: ParameterId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum PickupState {
    Disabled,
    Matched,
    Waiting {
        effective_position: NormalizedValue,
        signed_mismatch: f64,
    },
}
