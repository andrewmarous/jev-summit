# XDJ-AZ board events: visible deck and mixer inputs

Source: [archived top-view rendering](sources/xdj-az-top-image.md). This is a **proposed event inventory**, not a verified MIDI mapping or exhaustive hardware specification. An event means a user operates a visible control; availability, mode-dependent behavior, routing, and electronic observability remain untested. Record the target deck/channel, control, action, value or direction, and timestamp when implementing capture. Deck list below describes the left deck section; the right deck has corresponding controls. Mixer channel strips are ordered 3, 1, 2, 4 from left to right.

## Deck events (one deck)

- Jog wheel: rotate; touch/release top surface.
- PLAY/PAUSE: press/release; CUE: press/release.
- Tempo fader: move; TEMPO RESET: press.
- Performance pads 1–8: press/release.
- Pad modes: HOT CUE, BEAT LOOP, SLIP LOOP, BEAT JUMP: select; their secondary labeled functions are mode-dependent.
- Loop: IN/CUE, OUT, RELOOP/EXIT, 4 BEAT LOOP, 8 BEAT LOOP: press; BEAT JUMP left/right: press.
- TRACK SEARCH previous/next and SEARCH backward/forward: press/hold/release.
- DECK SELECT 1/3 (left section): press to select the controlled deck.
- JOG FEEL: turn; VINYL: toggle; VINYL SPEED ADJUST: turn.
- BEAT SYNC, KEY SYNC, MASTER TEMPO, SLIP, QUANTIZE: press/toggle.
- CUE/LOOP CALL previous/next, DELETE, MEMORY: press.
- PAD INFO DISPLAY: press.

## Mixer events (one channel)

- TRIM: turn.
- EQ/ISO HI, MID, LOW: turn each knob independently.
- COLOR: turn.
- Channel CUE: press/toggle headphone monitoring.
- Channel fader: move.

## Shared mixer events that can affect the channel

- Crossfader: move (effect depends on channel assignment; assignment controls are not visible in this view).
- SOUND COLOR FX: select SPACE, DUB ECHO, CRUSH, SWEEP, NOISE, or FILTER; PARAMETER: turn.
- BEAT FX channel selection: select 3, 1, 2, 4, MIC, CF A, CF B, or MST. The latter targets may affect the channel indirectly, depending on routing.
- BEAT FX effect selector: turn; X-PAD: touch/move/release; beat-step left/right: press; BEAT: press; TAP: tap; AUTO/TAP: press; FX QUANTIZE: toggle; TIME: turn; LEVEL/DEPTH: turn; ON/OFF: press/toggle.
- MASTER LEVEL and master EQ LOW, MID, HI: turn (affect the combined master output, not the isolated channel).
- Headphones MIX and LEVEL: turn; LINK CUE: press/toggle (affect monitoring, not necessarily master output).
- BOOTH LEVEL: turn (affects booth output, not necessarily master output).

Not included: deck-adjacent MIC controls, screen/touch UI and browsing controls, USB media controls, and hidden/rear-panel inputs. This inventory is limited to the deck and mixer controls identified in the rendering; do not infer a complete device event schema from it.
