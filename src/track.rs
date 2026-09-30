use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackState {
    pub asset_version: String,
    pub metadata: TrackMetadata,
    pub source: TrackSource,
    pub preparation: TrackPreparation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackMetadata {
    pub title: StateValue<String>,
    pub artist: StateValue<String>,
    pub album: StateValue<String>,
    pub additional: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackSource {
    pub kind: SourceKind,
    pub locator: StateValue<String>,
    pub duration: StateValue<Seconds>,
    pub sample_rate_hz: StateValue<u32>,
    pub channels: StateValue<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum SourceKind {
    AudioFile,
    Stream,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackPreparation {
    pub revision: String,
    pub analysis_revision: StateValue<String>,
    pub original_key: StateValue<MusicalKey>,
    pub key_analysis: StateValue<Vec<KeyRegion>>,
    pub beat_grid: StateValue<BeatGrid>,
    pub hot_cues: BTreeMap<CueId, SavedCue>,
    pub loops: BTreeMap<LoopId, SavedLoop>,
    pub markers: BTreeMap<MarkerId, PreparedMarker>,
    pub analyzed_gain: StateValue<Decibels>,
    /// Current edited preparation is above; this identifies unsaved edits.
    pub unsaved_edits: StateValue<PreparationEdits>,
    pub sections: StateValue<Vec<SectionAnnotation>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreparationEdits {
    pub base_revision: String,
    pub changed_fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyRegion {
    pub start: SourcePosition,
    pub end: StateValue<SourcePosition>,
    pub key: MusicalKey,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatGrid {
    pub revision: String,
    /// Each anchor starts a tempo segment; tempo is constant until the next anchor.
    pub anchors: Vec<BeatGridAnchor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatGridAnchor {
    pub position: SourcePosition,
    pub beat: Beats,
    pub tempo: Bpm,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedCue {
    pub slot: StateValue<u32>,
    pub position: Position,
    pub kind: CueKind,
    pub label: StateValue<String>,
    pub color: StateValue<Color>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum CueKind {
    HotCue,
    Memory,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedLoop {
    pub start: Position,
    pub end: Position,
    pub slot: StateValue<u32>,
    pub label: StateValue<String>,
    pub color: StateValue<Color>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreparedMarker {
    pub position: Position,
    pub kind: MarkerKind,
    pub label: StateValue<String>,
    pub color: StateValue<Color>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum MarkerKind {
    IntroStart,
    IntroEnd,
    OutroStart,
    OutroEnd,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SectionAnnotation {
    pub start: Position,
    pub end: Position,
    pub label: String,
}
