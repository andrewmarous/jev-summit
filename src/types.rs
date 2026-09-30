use serde::{Deserialize, Serialize};

macro_rules! ids {
    ($($name:ident),* $(,)?) => {$ (
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub String);
        impl From<&str> for $name {
            fn from(value: &str) -> Self { Self(value.to_owned()) }
        }
    )*};
}
ids!(
    TrackId,
    DeckId,
    ChannelId,
    BusId,
    InputId,
    EffectUnitId,
    EffectId,
    ControlSurfaceId,
    ControlId,
    CueId,
    LoopId,
    MarkerId,
    ParameterId,
    PlaybackInstanceId,
    SnapshotId,
    OperationId
);

/// Unset is an observed absence, not a missing observation or lack of support.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", content = "value", rename_all = "snake_case")]
pub enum StateValue<T> {
    Known(T),
    Unset,
    #[default]
    Unknown,
    Unsupported,
}

macro_rules! units {
    ($($name:ident),* $(,)?) => {$ (
        #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub f64);
    )*};
}
// Values must be finite for JSON. Range validation belongs at ingestion boundaries.
units!(
    Bpm,
    Beats,
    Seconds,
    Decibels,
    Semitones,
    PlaybackRate,
    NormalizedValue
);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourcePosition {
    pub frame: u64,
    pub sample_rate_hz: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeatGridRef {
    pub track: TrackId,
    pub revision: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MusicalPosition {
    pub grid: BeatGridRef,
    /// Fractional beat index in the referenced grid, not beat phase.
    pub beat: Beats,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "coordinate", content = "value", rename_all = "snake_case")]
pub enum Position {
    Source(SourcePosition),
    Musical(MusicalPosition),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterRange {
    pub min: f64,
    pub max: f64,
    pub step: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum ParameterUnit {
    Normalized,
    Decibels,
    Hertz,
    Seconds,
    Beats,
    Semitones,
    Ratio,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterState {
    pub value: StateValue<f64>,
    pub unit: ParameterUnit,
    pub range: StateValue<ParameterRange>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum EntityRef {
    Track(TrackId),
    Deck(DeckId),
    Channel(ChannelId),
    Bus(BusId),
    Input(InputId),
    EffectUnit(EffectUnitId),
    ControlSurface(ControlSurfaceId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum SignalSource {
    Deck(DeckId),
    Input(InputId),
    Bus(BusId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum EffectTarget {
    Deck(DeckId),
    Channel(ChannelId),
    Input(InputId),
    Bus(BusId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum OperatingMode {
    Performance,
    Preparation,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MusicalKey {
    /// Application-neutral key spelling, e.g. "F# minor".
    pub notation: String,
}
