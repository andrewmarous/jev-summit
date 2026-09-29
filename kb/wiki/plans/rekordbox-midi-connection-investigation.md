# Plan: investigate the Rekordbox MIDI connection

Status: **plan**. This plan covers the MIDI boundary only. It does not include CLM, audio capture, DJ policy, or application state estimation.

## Questions and evidence

| Question | Evidence required |
| --- | --- |
| Which OS MIDI endpoints appear when the FLX4 connects? | Endpoint names, directions, manufacturer, stable identifiers, and enumeration logs before and after connection. |
| Can a second process observe FLX4 input while Rekordbox runs? | Timestamped raw capture with Rekordbox open and closed. |
| Which input devices does Rekordbox accept? | MIDI device selector observations and a visible response to a message from each test device. |
| Does Rekordbox accept an IAC virtual source alongside the FLX4? | Mapping screen evidence plus a controlled UI response to a sent message. |
| Which bytes change a chosen Rekordbox field? | Captured bytes, mapping settings, sent bytes, and observed UI response. |
| Does Rekordbox emit any MIDI on an output endpoint? | Captured output events after a control change and after a UI-only change. |

An OS endpoint does not prove that Rekordbox listens to it. MIDI channels belong to messages, not ports. Device names alone do not identify stable endpoints across reconnects.

## Preparation

1. Record the macOS version, Rekordbox version, Performance mode, and FLX4 firmware version.
2. Export the current Rekordbox MIDI settings before changes. See the [MIDI Learn guide](../sources/rekordbox-official-docs.md).
3. Use a quiet test track and a low master level. A MIDI command can change the output level.
4. Set up a raw CoreMIDI monitor and sender. Use `uv` for any Python dependencies or scripts.
5. Record both the endpoint display name and OS identifier. Do not publish personal device identifiers.

Do not infer Rekordbox access from macOS endpoint discovery. Do not change driver settings or the Rekordbox database for this investigation.

## Phase 1: enumerate and receive

1. List all MIDI sources and destinations with the FLX4 disconnected.
2. Connect the FLX4. List the endpoints again.
3. Record the direction and properties of each new endpoint.
4. Open a monitor on each FLX4 source.
5. Turn the Deck 1 LOW EQ knob through its range. Save every raw byte and a monotonic timestamp.
6. Repeat the capture while Rekordbox runs. Record whether the monitor still receives the messages.
7. Repeat for Deck 2 LOW EQ, one button, and one indicator LED.

Compare the LOW EQ captures with the [vendor MIDI list](../flx4-midi-eq.md). The list publishes CC15 and CC47 as an MSB/LSB pair. Measure the actual channel, order, values, and update frequency. Record whether the OS or driver exposes multiple virtual ports.

## Phase 2: find a writable route into Rekordbox

1. Enable one macOS IAC bus in Audio MIDI Setup.
2. Enumerate the IAC endpoints in the test sender.
3. Open Rekordbox Performance mode and its MIDI settings window.
4. Record whether Rekordbox lists the IAC device alongside the FLX4.
5. If Rekordbox lists IAC, create one spare mapping for a low-risk visible function.
6. Send one note or CC message through IAC. Record the sender result and Rekordbox UI response separately.
7. Repeat with the FLX4 connected and disconnected.
8. Restore the original mapping after the test.

If Rekordbox does not list IAC, record the failure. Do not assume that USB equipment guidance guarantees virtual-device support. Investigate a supported external MIDI interface only after this result.

## Phase 3: encode messages for mixer controls

1. Inspect the [vendor list](../../raw/rekordbox/flx4-midi-messages-j1.pdf#page=2) and the active Rekordbox mapping for LOW EQ.
2. Record the exact MIDI IN code, deck, and knob resolution in Rekordbox.
3. If the mapping accepts a test source, send known LOW EQ positions at low audio volume.
4. Record the exact bytes, destination endpoint, send time, sender result, and UI position.
5. Test minimum, center, maximum, and small adjacent values. Do not jump directly to a loud setting.
6. Repeat each position after a physical FLX4 knob movement. Record any takeover behavior.

For 14-bit CC, test the MSB/LSB order and whether Rekordbox consumes both bytes. An OS send result means only that the transport accepted the message. Record audible results separately only if the test setup permits safe monitoring.

## Phase 4: inspect MIDI output from Rekordbox

1. Monitor each available destination route without sending test commands.
2. Change one mapped function from the Rekordbox UI.
3. Change the same function on the FLX4.
4. Record output events, timestamps, and their destination.
5. Repeat for an indicator function and LOW EQ.

The [MIDI Learn guide](../../raw/rekordbox/rekordbox-midi-learn-7.0.5.pdf#page=6) describes MIDI OUT for indicators. It does not promise LOW EQ state feedback. If no output appears, report the absence under the tested routes and conditions.

## Deliverables and decision

Save a port topology diagram, sanitized enumeration data, timestamped raw MIDI logs, and Rekordbox mapping exports. Add an evidence table with one row per tested route. Each row must distinguish `sent`, `OS received`, `Rekordbox UI changed`, and `Rekordbox MIDI output seen`.

The result must name a tested input route into Rekordbox or a specific blocker. It must also name each readable FLX4 source and any tested Rekordbox output route. Mark all untested endpoints as unknown. Do not describe a last sent value as actual Rekordbox state.

This plan narrows the earlier [broad probe plan](rekordbox-probe-v0.1.md). The [CLM-oriented proposal](midi-tool-interface.md) is not the scope of this investigation.
