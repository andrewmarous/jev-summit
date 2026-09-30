use crate::{timing::DeckTimingState, types::*};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DeckState {
    pub content: DeckContent,
    pub transport: TransportState,
    pub tempo_and_pitch: TempoPitchState,
    pub cue: CueExecutionState,
    pub looping: LoopExecutionState,
    pub slip: StateValue<SlipState>,
    pub quantize: QuantizeState,
    pub timing: DeckTimingState,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum DeckContent {
    #[default]
    Unloaded,
    Loading {
        requested_track: StateValue<TrackId>,
    },
    Ready {
        track: TrackId,
        instance: PlaybackInstanceId,
    },
    Error {
        requested_track: StateValue<TrackId>,
        message: String,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TransportState {
    pub mode: StateValue<TransportMode>,
    pub position: StateValue<SourcePosition>,
    pub direction: StateValue<PlaybackDirection>,
    pub at_end: StateValue<bool>,
    pub repeat: StateValue<RepeatMode>,
    /// Positive magnitude; direction is represented separately.
    pub effective_rate: StateValue<PlaybackRate>,
    pub activity: StateValue<TransportActivity>,
    pub ramp: StateValue<TransportRamp>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum TransportMode {
    Playing,
    Paused,
    Stopped,
    Other(String),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaybackDirection {
    Forward,
    Reverse,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum RepeatMode {
    Disabled,
    Track,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TransportActivity {
    Idle,
    Scratch { signed_rate: PlaybackRate },
    Search { signed_rate: PlaybackRate },
    Nudge { rate_offset: f64 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportRamp {
    pub kind: RampKind,
    pub target_rate: PlaybackRate,
    pub remaining: Seconds,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RampKind {
    Start,
    Stop,
    RateChange,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TempoPitchState {
    pub base_rate: StateValue<PlaybackRate>,
    pub control_range: StateValue<ParameterRange>,
    pub control_direction: StateValue<TempoControlDirection>,
    pub tempo_reset: StateValue<bool>,
    pub local_bpm: StateValue<Bpm>,
    pub effective_bpm: StateValue<Bpm>,
    pub pitch_adjustment: StateValue<Semitones>,
    pub effective_key: StateValue<MusicalKey>,
    pub key_lock: StateValue<bool>,
    pub key_lock_mode: StateValue<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TempoControlDirection {
    IncreasingSpeedsUp,
    IncreasingSlowsDown,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CueExecutionState {
    pub temporary_cue: StateValue<SourcePosition>,
    pub mode: StateValue<String>,
    pub hot_cue_bindings: BTreeMap<u32, CueId>,
    pub preview: StateValue<CuePreview>,
    pub held_cue: StateValue<HeldCue>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CuePreview {
    pub active_slot: StateValue<u32>,
    pub return_position: StateValue<SourcePosition>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeldCue {
    pub slot: StateValue<u32>,
    pub release_behavior: ReturnBehavior,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum ReturnBehavior {
    Continue,
    ReturnAndPause,
    ResumeUnderlying,
    Other(String),
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LoopExecutionState {
    pub boundaries: StateValue<LoopBoundaries>,
    pub enabled: StateValue<bool>,
    pub saved_loop: StateValue<LoopId>,
    pub selected_beat_size: StateValue<Beats>,
    pub anchor: StateValue<LoopAnchor>,
    pub edit_mode: StateValue<LoopEditMode>,
    pub roll: StateValue<LoopRoll>,
    pub selected_beat_jump: StateValue<Beats>,
    pub other_action_sizes: BTreeMap<String, Beats>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoopBoundaries {
    pub start: Position,
    pub end: Position,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum LoopAnchor {
    CurrentPosition,
    LoopStart,
    LoopEnd,
    Other(String),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum LoopEditMode {
    None,
    Defining,
    EditingStart,
    EditingEnd,
    Other(String),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoopRoll {
    pub size: Beats,
    pub return_behavior: ReturnBehavior,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum SlipState {
    Disabled,
    Armed,
    Running {
        underlying_position: SourcePosition,
        retained_rate: PlaybackRate,
        direction: PlaybackDirection,
        return_behavior: ReturnBehavior,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct QuantizeState {
    pub enabled: StateValue<bool>,
    pub interval: StateValue<Beats>,
    pub operation_settings: BTreeMap<String, QuantizeOperationSettings>,
    pub pending: StateValue<Vec<PendingQuantizedOperation>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuantizeOperationSettings {
    pub enabled: StateValue<bool>,
    pub interval: StateValue<Beats>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingQuantizedOperation {
    pub id: OperationId,
    pub operation: QuantizedOperation,
    pub target_boundary: MusicalPosition,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum QuantizedOperation {
    Play,
    Pause,
    TriggerCue { cue: CueId },
    EnableLoop { boundaries: LoopBoundaries },
    DisableLoop,
    BeatJump { distance: Beats },
    Other { name: String },
}
