# Performance application state

Status: **proposed state model**, based on the project discussion and inspection of the local Mixxx source tree on 2026-09-30.

## Purpose and boundaries

The runtime takes a snapshot of the performance and sends it to a decision model. This document defines the information in that snapshot.

Application state gives meaning to inputs. A button press alone does not describe playback, routing, or the resulting sound. Current state reflects the context established by earlier inputs.

The ordered-log analogy explains this distinction. It does not imply state replication. Input logs, transition functions, and capture mechanisms are outside this document.

This inventory extends the [XDJ-AZ board interactions](board-events.md). It does not establish which properties the XDJ-AZ or Rekordbox exposes.

## Entities and conventions

- **Track:** a musical asset and its prepared data.
- **Deck:** a playback instance of a track.
- **Channel:** a mixer input with a source and processing configuration.
- **Effect unit:** an effect or ordered effect chain with routing and parameters.
- **Bus:** a combined signal path, such as master, headphones, or booth.
- **Control context:** current modes that determine the meaning of an interaction.

A track can appear in multiple decks. Decks can have different positions and active loops for the same track. Channels do not necessarily correspond one-to-one with decks.

Collections use stable identifiers and references. The model does not assume four decks, eight pads, or three EQ bands.

Position values identify their coordinate system and units. Track positions can use source frames with a source sample rate. Musical positions reference a beat grid.

Unsupported, unknown, and unset values are distinct. An unset cue is not a cue at position zero. Capture limitations do not change the state definition.

## 1. Snapshot and setup

- Snapshot identifier and schema version.
- Capture time and the performance-time reference.
- Logical deck, channel, effect-unit, bus, and control-surface identifiers.
- Source-to-channel and channel-to-bus relationships.
- Operating mode and configuration that affects playback or control interpretation.
- Supported features and parameter ranges for each entity.
- Output sample rate and relevant output latency.

Capture time provides context for time-dependent values. Device connectivity and observation freshness belong to capture metadata, not core musical state.

## 2. Track state

For each referenced track:

### Identity and source

- Stable track identity and asset version.
- Title, artist, and other available identifying metadata.
- Duration, source sample rate, and available source channels.
- Source kind, such as an audio file or another supported asset.

### Musical preparation

- Original key and available key analysis.
- Beat-grid anchors, beat positions, and tempo changes.
- Beat-grid revision and relevant analysis revision.
- Saved hot cues: identifier, slot, position, type, label, and color.
- Saved loops: identifier, start, end, slot if applicable, label, and color.
- Available intro, outro, and other prepared markers.
- Track gain analysis, if the application uses it for automatic gain.
- Current edits to prepared data.

Phrase and section annotations are optional analysis extensions. The inspected controls do not establish their availability.

## 3. Deck state

For each logical deck:

### Loaded content

- Loaded track reference and playback-instance identity.
- Load status: unloaded, loading, ready, or error.
- Current load error, if applicable.

### Transport

- Transport mode: playing, paused, stopped, or another supported mode.
- Current source position.
- Playback direction.
- End-of-track status and repeat configuration.
- Effective playback rate.
- Current scratch, search, or temporary nudge activity.
- Relevant start, stop, or rate-ramp state.

### Tempo and pitch

- Base tempo adjustment.
- Tempo-control range and direction convention.
- Tempo-reset state.
- Local track BPM at the current position.
- Effective BPM after rate adjustment.
- Pitch adjustment and effective key.
- Key-lock/master-tempo state and relevant operating mode.

The base tempo adjustment and effective playback rate are separate values. Sync, scratching, reverse, and temporary nudges can change effective playback.

### Cue execution

- Current temporary cue position.
- Cue operating mode.
- Hot-cue bindings for this deck.
- Current cue-preview state and active cue slot, if applicable.
- Relevant held-cue behavior that affects release.

Saved track cues and the temporary deck cue are separate state. A cue-preview action can affect transport without changing a saved cue.

### Loop execution

- Current loop start and end.
- Loop enabled status.
- Reference to a saved loop, if the current loop uses one.
- Selected beat-loop size.
- Loop anchor configuration.
- Loop-definition or boundary-editing mode.
- Loop-roll activity and relevant return behavior.
- Selected beat-jump size.
- Other supported loop-action sizes.

Loop boundaries can exist while the loop is disabled. Selected beat-loop size does not imply that a loop is active.

### Slip and quantize

- Slip mode: disabled, armed, or running where supported.
- Underlying slip position.
- Retained slip rate and relevant return behavior.
- Quantize enabled status.
- Quantization interval and supported operation-specific configuration.
- Pending quantized operations and their target boundary, if the application retains them.

The audible position and underlying slip position can differ. Both positions affect the next action.

## 4. Musical timing and synchronization

### Per-deck timing

- Current beat position and beat index.
- Beat phase or distance from the beat boundary.
- Previous, next, and closest beat positions, where useful.
- Beat-grid reference for these values.

### Shared timing and relationships

- Sync membership and mode for each deck.
- Current sync leader.
- Internal/shared clock tempo and phase, where applicable.
- Current tempo and phase relationships between decks.
- Supported tempo-only versus phase-sync modes.
- Retained synchronization state that affects subsequent behavior.

Equal BPM does not imply equal beat phase. Track-local BPM, effective BPM, and shared clock BPM are distinct values.

## 5. Mixer channel state

For each channel:

- Assigned source and source kind.
- External-input or passthrough mode, where supported.
- Trim/input gain.
- Applied automatic gain or track-gain adjustment, where supported.
- EQ band identities, values, and ranges.
- EQ/isolator mode and relevant processing configuration.
- Channel fader value.
- Mute state.
- Crossfader assignment: A, B, or bypass.
- Routing to the master and other supported buses.
- Headphone cue selection.
- Talkover participation, where applicable.
- References to assigned effects and color-effect processing.

Playback and mixing remain separate. A playing deck can feed a closed channel, an excluded bus, or an external routing path.

## 6. Effect state

For each effect unit or chain:

- Unit identity and supported effect configuration.
- Selected effect identities and their order.
- Enabled or bypassed status for the unit and each effect.
- Target channels, inputs, or buses.
- Parameter identities, values, units, and ranges.
- Wet/dry value and mixing mode.
- Macro/superknob value and parameter assignments, where supported.
- Timing source: track, shared clock, or manual tempo.
- Beat division and time parameter.
- FX quantize state, where supported.
- Current pad or X-PAD interaction that affects the effect.
- Tail or residual-signal summary, if available.

The effect target belongs in state. The same effect configuration produces different results on a deck, microphone, or master bus.

Exact audio checkpoints require internal DSP memory. Delay buffers, filter histories, effect tails, and smoothing state are outside the default decision snapshot.

## 7. Shared mixer and output buses

### Crossfader

- Position.
- Curve/mode.
- Reverse state.
- Relevant calibration.

### Master

- Assigned sources and routing.
- Gain/level.
- EQ band values and configuration.
- Balance.
- Mono or other supported output modes.
- Enabled/muted status.
- Relevant output delay.

### Headphones

- Selected cue sources.
- Cue/master blend.
- Gain/level.
- Split-cue mode.
- Link-cue state, where supported.
- Enabled status and relevant output delay.

### Booth

- Source/routing configuration.
- Gain/level.
- Enabled/muted status.
- Relevant output delay.

### Microphones and auxiliary sources

- Source identity and routing.
- Enabled/muted status.
- Input gain and supported EQ.
- Talkover enabled status and ducking configuration.
- Current talkover/ducking activity.

Headphone monitoring and audience output are separate paths. The model needs both to reason about private preparation and public playback.

## 8. Interaction context

For each relevant control surface or software control context:

- Logical deck selected for each physical deck section.
- Selected pad mode and bank.
- Active modifiers, such as SHIFT.
- Held buttons whose release affects application behavior.
- Jog-touch state.
- Vinyl/jog operating mode.
- Jog-feel and vinyl-speed configuration, where they affect behavior.
- Selected effect unit, effect target, and parameter layer.
- Selected cue/loop entry for call, delete, or memory actions.
- Current display or screen mode, if it changes available actions.
- Browser selection and load target, if track selection is in scope.
- Relevant physical knob/fader positions.
- Pickup/soft-takeover state and mismatch with effective values.

Physical positions are relevant when they affect future interactions. They do not replace effective deck or mixer values.

A semantic action interface can omit irrelevant physical details. Application modes that alter semantic action behavior remain part of the snapshot.

## 9. Sound measurements attached to the snapshot

These measurements supplement application state. They do not replace playback, gain, or routing state.

- Per-source and per-channel signal levels.
- Master, headphone, and booth signal levels where available.
- Peak and clipping indicators, with measurement windows.
- Signal-presence measurements and their thresholds.
- Optional recent master and headphone audio references.
- Audio-window timestamps and alignment with snapshot time.

A playing deck and raised fader do not prove audible sound. Silence in the source or another processing stage can suppress output.

## 10. Derived decision summaries

These summaries are optional views of the state. They are not separate authoritative values.

- Time and beats remaining in each deck.
- Time or beats until relevant cues, loop boundaries, or markers.
- Current loop duration in time and beats.
- Tempo differences between decks.
- Beat-phase differences between decks.
- Current routing paths from sources to output buses.
- Effective gain-path summaries, where calculable.
- Available prepared cues and loops for the current track.
- Current physical-control targets and pad meanings.

Analysis-based summaries, such as phrase position, remain separate from exact transport values.

## Proposed snapshot structure

```text
PerformanceSnapshot
  identityAndTime
  setupAndCapabilities
  tracksById
  decksById
  synchronization
  mixer
    channelsById
    crossfader
    master
    headphones
    booth
    microphonesAndAuxiliarySources
  effectUnitsById
  interactionContextsById
  soundMeasurements
  derivedDecisionSummaries
```

The first schema can group fields differently without changing these distinctions. The decision model needs current behavior, available musical material, and context for its next action.

## Sources and evidence limits

The following local Mixxx files informed this proposal. Source inspection is not a hardware observation or proof of XDJ-AZ readback.

- [Engine control frame information](../../research/mixxx/src/engine/controls/enginecontrol.h): source positions, source sample rate, and engine control separation.
- [Stored cue objects](../../research/mixxx/src/track/cue.h): cue type, boundaries, hot-cue index, label, and color.
- [Cue controls](../../research/mixxx/src/engine/controls/cuecontrol.cpp): temporary cue, cue mode, and interactions with loops.
- [Loop controls](../../research/mixxx/src/engine/controls/loopingcontrol.cpp): loop boundaries, enabled status, loop size, anchor, and beat-jump size.
- [Rate controls](../../research/mixxx/src/engine/controls/ratecontrol.cpp): rate ratio, base rate, range, reverse, jog, scratch, and temporary adjustments.
- [Key controls](../../research/mixxx/src/engine/controls/keycontrol.cpp): source key, effective key, pitch adjustment, and key-lock modes.
- [BPM controls](../../research/mixxx/src/engine/controls/bpmcontrol.cpp): local BPM, beat distance, and sync relationships.
- [Quantize controls](../../research/mixxx/src/engine/controls/quantizecontrol.cpp): quantize state and neighboring beat positions.
- [Deck engine](../../research/mixxx/src/engine/enginebuffer.h) and [slip modes](../../research/mixxx/src/engine/slipmodestate.h): separate slip position, retained rate, and slip lifecycle.
- [Sync controls](../../research/mixxx/src/engine/sync/synccontrol.cpp): sync mode, leadership, tempo sync, and phase sync.
- [Engine channels](../../research/mixxx/src/engine/channels/enginechannel.cpp): headphone cue, master routing, crossfader orientation, and talkover.
- [Engine mixer](../../research/mixxx/src/engine/enginemixer.cpp): output gains, crossfader configuration, headphone blend, delays, and output modes.
- [Effect chains](../../research/mixxx/src/effects/effectchain.h): effect routing, chain mixing mode, and macro controls.
- [Soft takeover](../../research/mixxx/src/controllers/softtakeover.h): physical-control pickup context.

The local research checkout is necessary to resolve these source links. The inventory combines Mixxx concepts, XDJ-AZ control labels, and proposed device-neutral extensions.
