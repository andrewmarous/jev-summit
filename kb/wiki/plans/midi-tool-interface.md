# Proposal: MIDI control and observation tool v0.1

Status: **proposal**, not an implemented or tested interface. Sources: [official documents](../sources/rekordbox-official-docs.md); test with [probe plan](rekordbox-probe-v0.1.md).

## Single-iteration contract

Expose one `dj_cycle` call that takes an observation request and optional bounded actions, and returns a timestamped snapshot plus results. The model must receive available information in one response; the tool must not claim that a sent command proves Rekordbox changed state.

```json
{
  "actions": [
    {"target":"deck1.eq.low", "value":0.5, "expected_revision":12},
    {"target":"deck2.cue", "operation":"set", "value":true}
  ],
  "observe": ["midi", "controls", "audio_levels", "routing", "capabilities"],
  "settle_ms": 50,
  "timeout_ms": 250
}
```

Response shape:

```json
{
  "time_monotonic_ns": 123456789,
  "snapshot_revision": 13,
  "capabilities": {"midi_output":true,"midi_input":true,"eq_readback":"unknown","cue_audio_capture":false},
  "routing": {"input_port":"configured-id", "output_port":"configured-id", "mapping_version":"local-v1"},
  "actions": [{"target":"deck1.eq.low","sent":true,"sent_at_ns":123456700,"acknowledged":false,"error":null}],
  "controls": {"deck1.eq.low":{"value":0.5,"source":"commanded","observed_at_ns":123456700,"revision":13}},
  "midi_events": [],
  "audio_levels": {"master":{"available":false},"cue":{"available":false}},
  "warnings": ["No Rekordbox state readback verified"]
}
```

`controls` values always carry provenance: `commanded`, `midi_feedback`, `hardware_input`, or `measured`. Never promote `commanded` to measured state. `expected_revision` prevents decisions based on a stale local snapshot; revision is **tool-local**, not a Rekordbox revision. Report timeouts and partial failures per action. Return current capabilities and device connection health even when no actions are sent.

## Adapter boundary

- A configurable mapping translates semantic targets to MIDI port, channel, CC/note, encoding, range and polarity. The [vendor MIDI table](../flx4-midi-eq.md) publishes LOW EQ CC15/CC47 for deck 1/2; load those as a versioned candidate mapping, then verify against hardware before enabling writes. For a 14-bit CC pair, send MSB then LSB as configured; clamp and rate-limit values, with no undocumented inference about Rekordbox EQ gain in dB.
- One MIDI backend enumerates ports, opens a virtual output toward Rekordbox and an input for incoming hardware/feedback messages. Detect disconnection; timestamp and deduplicate events. Verify that the virtual port is accepted by the installed Rekordbox version.
- Optional audio backend supplies separately identified master/cue level measurements only if independently routed and captured. Levels are measurements of signal, not proof of EQ state; no cue data if only master is available.
- Store configuration and calibration outside the model call; return mapping version and capability diagnostics in every cycle. Allowlist controls; disallow bulk resets and unsafe jumps; rate-limit and provide an emergency stop/human override.

## Before implementation

Verify the vendor-published FLX4 MIDI bytes with a MIDI monitor; determine whether Rekordbox accepts the virtual sender alongside the FLX4; probe mapping, feedback, cue capture and latency. Until then, a cycle can report commands sent and locally received MIDI but cannot provide complete actual Rekordbox state. See [system model](../system-model.md).
