# Rekordbox and DDJ-FLX4 official documentation

Downloaded 2026-09-27 and 2026-09-28: [source manifest and original URLs](../../raw/rekordbox/SOURCES.md).

- [FLX4 MIDI message list (Japanese)](../../raw/rekordbox/flx4-midi-messages-j1.pdf): page 2 mixer table lists paired CC15 (MSB) and CC47 (LSB) for deck 1 and deck 2 EQ LOW. The PDF's text extraction scrambles table columns; verify visually and on the target hardware.
- [MIDI Learn guide](../../raw/rekordbox/rekordbox-midi-learn-7.0.5.pdf): reference for supported MIDI assignment workflow; a mapping is not proof of state feedback.
- [Rekordbox manual 7.1.4](../../raw/rekordbox/rekordbox-manual-7.1.4.pdf): version-specific reference for mixer, audio and application configuration. Confirm behavior against installed version.
- [FLX4 hardware diagram](../../raw/rekordbox/flx4-hardware-diagram.pdf): control labels and default assignments, not DSP implementation.
- FAQ snapshots: [operation hints](../../raw/rekordbox/faq-operation-hints-7.html), [Ableton Link](../../raw/rekordbox/faq-ableton-link.html), [connecting DJ units](../../raw/rekordbox/faq-connecting-to-dj-units.html), [troubleshooting](../../raw/rekordbox/faq-trouble-shooting.html). Full-page snapshots; answers may change online.

**Limits:** The English MIDI list returned HTTP 403, but the Japanese vendor PDF downloaded successfully. Its published message assignments are documentation, not an observation of this machine. No runtime Rekordbox MIDI or audio observations were made. The MIDI Learn guide (pp. 5–6) documents optional MIDI OUT indicator messages; this is not general parameter-state feedback.
