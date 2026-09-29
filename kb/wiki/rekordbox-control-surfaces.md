# Rekordbox software interfaces and control surfaces

These are **documented interfaces**, not measurements of the target installation. Sources: [official docs](sources/rekordbox-official-docs.md).

## Live MIDI control

The [MIDI Learn guide](../raw/rekordbox/rekordbox-midi-learn-7.0.5.pdf#page=2) says MIDI Learn assigns Performance-mode functions to equipment controls. Its [setting table](../raw/rekordbox/rekordbox-midi-learn-7.0.5.pdf#page=5) includes function, deck, MIDI IN and control type; knob/slider types support 0–127 or 0–16383 values. The guide documents device selection, learning, editing and importing/exporting mappings ([pp. 4–8](../raw/rekordbox/rekordbox-midi-learn-7.0.5.pdf#page=4)). It says the feature is intended for AlphaTheta products and that third-party product configuration is not supported; the guide calls for USB-connected equipment. Thus acceptance of a virtual IAC sender remains a **test question**, not a guarantee.

The guide also defines [MIDI OUT](../raw/rekordbox/rekordbox-midi-learn-7.0.5.pdf#page=6): indicator functions (such as lights) can send MIDI to equipment. This does **not** document general EQ/fader state readback. See the [probe plan](plans/rekordbox-probe-v0.1.md).

## Audio, library and synchronization

The [manual's Audio settings](../raw/rekordbox/rekordbox-manual-7.1.4.pdf#page=229) distinguish internal mixing (rekordbox mixer; master/headphones/booth output channels) from external mixing (separate deck, sampler and preview outputs; pp. 229–230). Its [Mixer settings](../raw/rekordbox/rekordbox-manual-7.1.4.pdf#page=235) offer EQ or ISOLATOR and an EQ Type when EQ is selected. Neither source specifies the DSP filter coefficients or a plugin interface.

The [operation FAQ](../raw/rekordbox/faq-operation-hints-7.html) and [manual](../raw/rekordbox/rekordbox-manual-7.1.4.pdf#page=245) describe File → Export Collection in xml format. This exports collection data, not live parameters. The [Ableton Link FAQ](../raw/rekordbox/faq-ableton-link.html) describes timing/BPM synchronization; controlling Link BPM via hardware requires a MIDI mapping to its subscreen, not the ordinary tempo slider. Link is not an audio or EQ transport.

For FLX4 streaming, the [troubleshooting FAQ](../raw/rekordbox/faq-trouble-shooting.html) describes routing PC MASTER OUT to computer speakers for OBS capture. That procedure documents master output, **not** an independently capturable headphone cue. The [connecting-units FAQ](../raw/rekordbox/faq-connecting-to-dj-units.html) includes mobile/iOS FLX4 USB and Bluetooth instructions; do not apply mobile limitations to desktop Rekordbox without a separate source.

## Boundary for an agent

A MIDI sender can issue mapped commands; a MIDI receiver may capture hardware messages and indicator outputs. Neither by itself establishes the actual current Rekordbox EQ state. Expose command provenance, tested capabilities and available audio measurements separately in the [one-call tool](plans/midi-tool-interface.md).
