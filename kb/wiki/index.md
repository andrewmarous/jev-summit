# Wiki index

Start here for queries. See [the schema](../AGENTS.md) for maintenance rules and [the log](log.md) for changes.

## Sources

- [Project conversation](sources/project-conversation.md) — user goals and current constraints; [raw notes](../raw/project-conversation-2026-09-27.md).
- [Karpathy: LLM Wiki](sources/karpathy-llm-wiki.md) — source for this knowledge-base pattern; [raw snapshot](../raw/karpathy-llm-wiki.md).
- [Rekordbox official documents](sources/rekordbox-official-docs.md) — downloaded MIDI list, manuals, diagram and FAQ snapshots; [source manifest](../raw/rekordbox/SOURCES.md).
- [Talker–Reasoner paper](sources/talker-reasoner.md) — language-model fast responder and asynchronous belief updates; [PDF](../raw/system-one/talker-reasoner-2410.08328.pdf).
- [DPT-Agent paper](sources/dpt-agent.md) — FSM/code-as-policy fast path and asynchronous reasoning; [PDF](../raw/system-one/dual-process-collaboration-2502.11882.pdf).
- [CLM repository](sources/clm.md) — contrastive state/action scoring, text-only v0.1; [README snapshot](../raw/system-one/clm-repository-readme.md).
- [XDJ-AZ top-view image](sources/xdj-az-top-image.md) — visual reference for deck, mixer, and EQ control positions; [archived JPEG](../raw/xdj-az/xdj-az-cgi-top.jpg).
- [Standalone hardware architecture](sources/standalone-hardware-architecture.md) — processor and OS reports for XDJ and PRIME 4 units; [research links](../raw/standalone-hardware-architecture-2026-09-29.md).

## Rekordbox interfaces

- [Control surfaces](rekordbox-control-surfaces.md) — MIDI Learn, audio routing, collection XML, Ableton Link and interface limits.
- [FLX4 LOW EQ MIDI](flx4-midi-eq.md) — vendor-published CC pair and end-to-end conceptual path.

## Project concepts

- [Vision and boundaries](vision.md) — Jev at a DJ board, including private cue selection.
- [System model](system-model.md) — proposed control and observation paths, with open questions.
- [XDJ-AZ board events](board-events.md) — proposed event inventory for one deck, one mixer channel, and shared mixer controls visible in the rendering.
- [Performance application state](performance-state.md) — proposed decision-model snapshot inventory, informed by Mixxx: tracks, decks, timing, mixer routing, effects, monitoring, and interaction context.
- [System 1 inference](system-one-inference.md) — three fast-path architectures and Jev's open latency measurements.
- [Low-memory standalone deployment](standalone-deployment.md) — hardware evidence, Rust design proposal, and deployment questions.

## Work

- [Rekordbox MIDI connection investigation](plans/rekordbox-midi-connection-investigation.md) — port discovery, input mapping, message encoding, and output observation.
- [Rekordbox probe v0.1 plan](plans/rekordbox-probe-v0.1.md) — broader experiment scope, steps, and success criteria.
- [XDJ-AZ EQ recording experiment](plans/xdj-az-eq-recording-experiment.md) — local port-discovery and MIDI/audio recorder, isolated EQ mapping tests in standalone mode.
- [MIDI tool interface proposal](plans/midi-tool-interface.md) — one-call action/observation contract with explicit provenance and limitations.
- [Probe results template](probe-results/template.md) — evidence matrix for the actual Rekordbox setup.
