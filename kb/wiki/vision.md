# Vision and boundaries

Source: [project conversation](sources/project-conversation.md).

**Decision:** Jev acts as a DJ at a board. It gets access to typical controls: transport, hot cues, loops, tempo, channel faders, crossfader, EQ, Color FX, Beat FX, and headphone cue selection. Cue selection changes what the listener hears without changing the audience master.

**Observed:** Jev is text-only today. A separate multimodal audio system is necessary for autonomous listening. No listener is selected yet.

**Decision:** Build and probe the board interface first. Do not reduce Jev to a single high-level `transition` command. Higher-level automation can supplement direct controls later.

**Open:** Which inputs can we capture independently from Rekordbox? Which controls accept external input? Which controls report actual state? See the [probe plan](plans/rekordbox-probe-v0.1.md).
