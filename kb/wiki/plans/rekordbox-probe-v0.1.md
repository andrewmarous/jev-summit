# Plan: Rekordbox probe v0.1

Context: [project conversation](../sources/project-conversation.md) and [system model](../system-model.md). This is a plan, not an observed compatibility result.

## Goal

Measure the controls, readback, and audio paths available on the user's Rekordbox setup. Produce evidence, not a production DJ agent. A successful probe identifies a viable path or documents a specific blocker.

## Scope

Use two local test tracks, a macOS IAC virtual MIDI bus, a small local MIDI sender/receiver, and manual observations. Use standard macOS audio tools or an available loopback device for audio tests. Do not install kernel drivers, modify Rekordbox internals, scrape its database, or automate its UI in v0.1.

## Prerequisites

1. Record the macOS version and the Rekordbox version, plan, and mode.
2. Record the controller, mixer, audio interface, and output routing. Record whether the application and hardware are on this machine.
3. Prepare two tracks with distinct audio and a safe monitoring level. Use local files that you can play legally.
4. Back up or export existing Rekordbox MIDI mappings before changes. Use a separate test mapping where possible.
5. Identify the audio capture device before the cue test. If no independent cue path exists, record that limit.

## Implementation steps

1. Create an IAC bus in Audio MIDI Setup. Make the bus visible to Rekordbox and the probe process.
2. Write a minimal probe utility to list MIDI ports, send note and CC messages, and log incoming MIDI with timestamps.
3. Log the exact message, channel, value, and target mapping for each test. Avoid hard-coded mappings without a mapping file.
4. Map one control at a time in Rekordbox. Test deck A play/pause, hot cue 1, channel fader, low EQ, crossfader, deck B headphone cue, and one FX control.
5. For each control, send an action. Record the UI response and audible result separately. Restore the initial state afterward.
6. For each control, move it in the UI or on hardware. Record whether the probe receives state feedback. Repeat after a manual change.
7. Play A to master and B to cue. Record independent audio streams when routing permits. Switch cue selection. Record whether the private stream changes while master stays unchanged.
8. Repeat a small set of actions ten times. Log command-to-UI or command-to-audio delays only when timestamps support measurement. Record missed actions.
9. Disconnect the probe during playback. Record what happens. Test human override and recovery at a safe level.
10. Fill in a [results sheet](../probe-results/template.md) with evidence and a next-step recommendation.

## Deliverables

- A runnable local probe utility with setup and cleanup instructions.
- A mapping file for tested MIDI messages. Do not commit personal device identifiers.
- Timestamped MIDI logs and an evidence index. Record audio clips only with permission.
- A completed results sheet for the actual machine and routing.

## Acceptance criteria

- The results distinguish command sent, UI response, audible response, and actual state readback.
- The results include at least one control from transport, mixer, cue, and FX, or name the mapping blocker.
- The cue test reports whether master and private cue audio are independently capturable.
- Every untested item has an explicit reason. No blank cell implies success.
- The recommendation names the next interface to build or the blocking capability to investigate.

## Limits and safety

CAUTION: Use low speaker volume before sending MIDI. A fader or FX change can cause an abrupt output change.

Do not assume that Rekordbox emits MIDI feedback because it accepts MIDI input. Do not call a recalled value actual state. If separate cue capture fails, keep it as a result rather than simulating success with the master feed.
