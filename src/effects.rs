use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectUnitState {
    pub enabled: StateValue<bool>,
    pub bypassed: StateValue<bool>,
    /// Vector order is processing order, independent of effect identity.
    pub chain: Vec<EffectState>,
    pub targets: Vec<EffectTarget>,
    pub wet_dry: StateValue<NormalizedValue>,
    pub mixing_mode: StateValue<EffectMixingMode>,
    pub macro_control: StateValue<EffectMacro>,
    pub timing: EffectTiming,
    pub quantize: StateValue<bool>,
    pub pad_interaction: StateValue<EffectPadInteraction>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectState {
    pub id: EffectId,
    pub implementation: String,
    pub enabled: StateValue<bool>,
    pub bypassed: StateValue<bool>,
    pub parameters: BTreeMap<ParameterId, ParameterState>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum EffectMixingMode {
    Insert,
    Send,
    Other(String),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectMacro {
    pub value: StateValue<NormalizedValue>,
    pub assignments: Vec<MacroAssignment>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MacroAssignment {
    pub effect: EffectId,
    pub parameter: ParameterId,
    pub output_range: ParameterRange,
    pub mapping: StateValue<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectTiming {
    pub source: StateValue<EffectTimingSource>,
    pub beat_division: StateValue<Beats>,
    pub time: StateValue<Seconds>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum EffectTimingSource {
    Deck(DeckId),
    SharedClock,
    Manual(Bpm),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectPadInteraction {
    pub held: bool,
    pub x: StateValue<NormalizedValue>,
    pub y: StateValue<NormalizedValue>,
    pub pressure: StateValue<NormalizedValue>,
}
