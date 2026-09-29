# Rekordbox LOW EQ experiment

This utility attempts to change one mapped LOW EQ control through MIDI. It does not read Rekordbox state or automate its interface.
The [investigation plan](../../kb/wiki/plans/rekordbox-midi-connection-investigation.md) defines the evidence requirements.

## Preparation

1. Record the macOS version, Rekordbox version, plan, mode, hardware, and audio routing.
2. Export the existing Rekordbox MIDI mapping.
3. Enable an IAC bus in macOS Audio MIDI Setup.
4. Open Rekordbox Performance mode and its MIDI settings.
5. Record whether Rekordbox lists the IAC device.
6. If IAC is absent, record the blocker and stop.
7. Create a separate LOW EQ mapping for the IAC device.
8. Copy the candidate JSON file to a local mapping file.
9. Set the channel, encoding, and controllers to match the active mapping.

The candidate contains published FLX4 Deck 1 assignments, not verified IAC assignments. Deck 2 uses channel 2 in the published candidate.
A `cc7` mapping uses `cc` instead of `msb_cc` and `lsb_cc`.
The values represent controller positions, not decibels. The numerical midpoint does not prove unity gain.

## Run

Run these commands from this directory. Dependencies use `uv` script metadata.

```sh
uv run probe.py ports
uv run probe.py send --mapping flx4-low-candidate.json --value 8192
uv run probe.py monitor --port 'EXACT INPUT NAME' --seconds 30 --log baseline.jsonl
```

CAUTION: Reduce the speaker volume before any live send. An EQ change can cause an abrupt output change.

After you verify the mapping, send one position:

```sh
uv run probe.py send --mapping local-mapping.json --value 8192 \
  --port 'EXACT OUTPUT NAME' --log send-center.jsonl \
  --execute --safe-level-confirmed
```

Each command sends one value only. It does not perform an automatic sweep or restore an unknown initial value.
The log records raw bytes and host timestamps. Receive timestamps describe polling time, not hardware arrival time.
Endpoint names do not provide stable CoreMIDI identifiers. Record those separately when required by the investigation plan.

1. Record the initial UI position before each send.
2. Record the resulting UI position after each send.
3. Record the audible response separately.
4. Restore the initial position manually.
5. Repeat with minimum, midpoint, adjacent values, and maximum at a safe level.
6. Repeat after a physical knob movement to inspect takeover behavior.
7. Repeat with the controller connected and disconnected.
8. For an available feedback route, monitor during UI-only and hardware changes.

An IAC monitor can receive the probe's own messages. Such messages do not prove Rekordbox feedback.
A successful send proves transport acceptance only. No received messages means no feedback on that tested route during that interval.

## Evidence and cleanup

Save a local table with these columns:

| Run/log | Mapping | Requested value | Transport accepted | UI before/after | Audible response | Feedback route/events | Blocker |
| --- | --- | --- | --- | --- | --- | --- | --- |
| | | | | | | | |

Use `unknown` for unobserved results. Preserve mapping exports and the endpoint topology beside the logs.
Do not publish personal endpoint names or identifiers without sanitization.

1. Restore the original Rekordbox MIDI mapping.
2. Restore the initial EQ position manually.
3. Disable the test IAC bus if it is no longer necessary.

## Offline tests

```sh
python3 -m unittest discover -s . -p 'test_*.py'
```

Hardware compatibility and actual EQ changes remain untested until the manual experiment completes.
