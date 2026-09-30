use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MixerState {
    pub channels: BTreeMap<ChannelId, ChannelState>,
    pub crossfader: CrossfaderState,
    pub buses: BTreeMap<BusId, BusState>,
    pub inputs: BTreeMap<InputId, InputState>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChannelState {
    pub source: StateValue<SignalSource>,
    pub input_mode: StateValue<InputMode>,
    pub trim: StateValue<Decibels>,
    pub automatic_gain: StateValue<Decibels>,
    pub eq: EqState,
    pub fader: StateValue<NormalizedValue>,
    pub muted: StateValue<bool>,
    pub crossfader_assignment: StateValue<CrossfaderAssignment>,
    pub routes: Vec<BusRoute>,
    pub headphone_cue: StateValue<bool>,
    pub talkover_participant: StateValue<bool>,
    pub effects: Vec<EffectUnitId>,
    pub color_effect: StateValue<EffectUnitId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum InputMode {
    Internal,
    External,
    Passthrough,
    Other(String),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrossfaderAssignment {
    A,
    B,
    Bypass,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusRoute {
    pub destination: BusId,
    pub enabled: StateValue<bool>,
    pub gain: StateValue<Decibels>,
}
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EqState {
    pub mode: StateValue<EqMode>,
    pub bands: BTreeMap<ParameterId, EqBand>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum EqMode {
    Equalizer,
    Isolator,
    Other(String),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EqBand {
    pub label: String,
    pub gain: ParameterState,
    pub center_frequency_hz: StateValue<f64>,
    pub killed: StateValue<bool>,
}
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CrossfaderState {
    pub position: StateValue<NormalizedValue>,
    pub curve: StateValue<String>,
    pub reversed: StateValue<bool>,
    pub calibration: StateValue<ParameterRange>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusState {
    pub sources: Vec<BusFeed>,
    pub gain: StateValue<Decibels>,
    pub enabled: StateValue<bool>,
    pub muted: StateValue<bool>,
    pub output_delay: StateValue<Seconds>,
    pub processing: BusProcessing,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusFeed {
    pub source: BusFeedSource,
    pub enabled: StateValue<bool>,
    pub gain: StateValue<Decibels>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum BusFeedSource {
    Channel(ChannelId),
    Input(InputId),
    Bus(BusId),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "settings", rename_all = "snake_case")]
pub enum BusProcessing {
    Master(MasterSettings),
    Headphones(HeadphoneSettings),
    Booth,
    Other {
        name: String,
        parameters: BTreeMap<ParameterId, ParameterState>,
    },
}
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MasterSettings {
    pub eq: EqState,
    pub balance: StateValue<NormalizedValue>,
    pub output_mode: StateValue<OutputMode>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum OutputMode {
    Stereo,
    Mono,
    Other(String),
}
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HeadphoneSettings {
    pub selected_cues: Vec<BusFeedSource>,
    pub cue_master_blend: StateValue<NormalizedValue>,
    pub split_cue: StateValue<bool>,
    pub link_cue: StateValue<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputState {
    pub kind: InputKind,
    pub enabled: StateValue<bool>,
    pub muted: StateValue<bool>,
    pub gain: StateValue<Decibels>,
    pub eq: EqState,
    pub routes: Vec<BusRoute>,
    pub talkover: StateValue<TalkoverState>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum InputKind {
    Microphone,
    Auxiliary,
    External,
    Other(String),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TalkoverState {
    pub enabled: StateValue<bool>,
    pub active: StateValue<bool>,
    pub threshold: StateValue<Decibels>,
    pub ducking_gain: StateValue<Decibels>,
    pub attack: StateValue<Seconds>,
    pub release: StateValue<Seconds>,
    pub targets: Vec<BusId>,
}
