# Plan: XDJ-AZ EQ recording experiment

Status: **plan**, not a hardware observation. The [AlphaTheta XDJ-AZ MIDI message list](https://downloads.support.alphatheta.com/software_info/all-in-one-dj-systems/XDJ-AZ/XDJ-AZ_MIDI_Message_List_E1.pdf) is a candidate mapping, not proof of MIDI output during standalone playback. This experiment is separate from the [Rekordbox probe](rekordbox-probe-v0.1.md). The [archived top-view image](../sources/xdj-az-top-image.md) shows the four channel EQ strips (3, 1, 2, 4 from left to right) and separate master EQ controls; it does not show computer connections or prove MIDI behavior.

## Goal and scope

Build a local computer app that discovers MIDI ports, records raw MIDI and audio, and helps identify messages for the LOW, MID, and HI EQ knobs on one AZ mixer channel. Test while playing from USB media on the AZ in standalone mode. Do not assume that the USB computer connection exposes mixer positions or standalone audio. The output is an evidence-backed mapping and synchronized recording, or a documented blocker. Do not build a training pipeline or control the AZ in this experiment.

## Preparation

1. Record the date, AZ firmware, computer OS, connection type, playback mode, channel source, and audio routing.
2. Get a track that you can record legally. Set the speaker and recorder levels low.
3. Connect the AZ computer USB-C port directly to the computer. Avoid DJ software initially.
4. Identify an audio input for the master signal. If the AZ exposes no usable standalone input, use a suitable audio interface connected to an AZ output. Do not connect a speaker-level output to a line input.
5. Keep the published MIDI list and its version or download date beside the experiment notes. Do not copy FLX4 assignments to the AZ.

## Probe app: required functions

- List MIDI input and output ports with the OS name and stable identifiers where available. Show connection and disconnection events. Permit explicit input selection and an optional all-input discovery mode. A PDF message list defines message assignments, not OS ports.
- Log every received MIDI packet before decoding. Save monotonic receive time in nanoseconds, wall-clock start time, port, status, channel, data bytes, and raw bytes. Preserve unknown messages, including possible high-resolution CC pairs and notes. Do not send MIDI to the AZ.
- Offer labeled manual markers for channel, LOW/MID/HI, movement start/end, direction, and approximate knob position. Timestamp markers on the same monotonic clock as MIDI.
- Record master audio as uncompressed PCM/WAV with sample rate, channel count, input name, frame count, and timestamp or frame index for each capture block. Report dropped blocks and overruns. Keep audio capture and MIDI logging active together.
- Save a session manifest with device identifiers, selected ports, settings, clock information, and file names. Save raw streams before making interpreted state-action pairs.
- Provide a replay report that groups messages around each marker. Compare message type, status/channel, controller or note, value range, direction, and repeats. Keep a versioned candidate mapping with an evidence reference for each assignment.

## Procedure

1. Start the app before connecting the AZ. Save the initial port list.
2. Connect the AZ. Save the new port list and select its candidate input. If no port appears, record that result and stop the MIDI test.
3. Start playback from USB media. Record 30 seconds with no EQ movement to measure background messages.
4. On one mixer channel, mark and move LOW only: slow full sweep, return, fast sweep, and three held positions. Repeat for MID and HI. Leave other controls untouched during each labeled segment.
5. Repeat the knob sequence three times. Then repeat one LOW sweep with playback stopped. Compare messages from playback and stopped modes.
6. Record master audio throughout a take. Mark a short, clear EQ movement near the start and end while audio plays. Record any audible change separately from the MIDI observation.
7. Disconnect and reconnect the computer USB cable after saving the take. Record whether the app detects the port change and recovers without mislabeling a new port.
8. Review raw logs against markers and the published MIDI list. For each knob, document candidates and competing explanations. Re-test any ambiguous assignment by moving only that knob.

## Analysis and acceptance

- **MIDI pass:** Each knob has a repeatable, distinguishable event pattern during standalone playback. Map raw bytes to a knob only after isolated repetitions support the assignment.
- **Audio pass:** The app saves master audio without reported drops. A captured master signal is not a measurement of an individual deck or cue signal.
- **Alignment pass:** MIDI, markers, and audio share a recorded clock relation. State the measured or estimated timing uncertainty. Do not infer precise audio-to-MIDI latency from an EQ change alone without a clear audio landmark.
- **Partial:** Report which knobs or modes produce events, and which do not. A position is unknown until a relevant event establishes it. A movement event does not prove that every knob reports its position at connection time.
- **Blocker:** If standalone EQ movement produces no computer-visible MIDI, preserve the port and negative-test evidence. Computer/DJ-software mode is a separate follow-up experiment, not evidence for standalone behavior.

## Deliverables

A runnable probe app with setup instructions; raw MIDI log; WAV file; marker log; session manifest; candidate mapping with confidence and supporting event ranges; and a short results note. Record hardware results as dated probe evidence before calling them observed facts. Do not commit copyrighted audio or machine-specific device identifiers to the repository.
