# Wiki log

Append new entries. Use `## [YYYY-MM-DD] operation | subject` headings.

## [2026-09-27] ingest | Project conversation and Karpathy LLM Wiki

Added [conversation notes](../raw/project-conversation-2026-09-27.md) and a [gist snapshot](../raw/karpathy-llm-wiki.md). Created [source notes](sources/project-conversation.md) and [pattern notes](sources/karpathy-llm-wiki.md). Integrated them into [vision](vision.md), [system model](system-model.md), and the [probe plan](plans/rekordbox-probe-v0.1.md). Created the [index](index.md). No Rekordbox tests have run.

## [2026-09-27] ingest | Rekordbox documentation and MIDI interface proposal

Downloaded official manual, MIDI Learn guide, FLX4 hardware diagram and four FAQ page snapshots; see [manifest](../raw/rekordbox/SOURCES.md) and [source note](sources/rekordbox-official-docs.md). Added [one-call MIDI interface proposal](plans/midi-tool-interface.md) and updated [index](index.md). English vendor MIDI message list returned HTTP 403; no hardware tests or actual-state readback confirmed.

## [2026-09-28] ingest | Japanese FLX4 MIDI list and source synthesis

Downloaded [Japanese vendor MIDI message list](../raw/rekordbox/flx4-midi-messages-j1.pdf) from the user-provided URL. Used uv to extract text from the downloaded PDFs and FAQ HTML for source review (temporary extracted text is not part of the raw archive). Updated [source note](sources/rekordbox-official-docs.md) and [manifest](../raw/rekordbox/SOURCES.md). Added synthesized [FLX4 LOW EQ](flx4-midi-eq.md) and [Rekordbox interfaces](rekordbox-control-surfaces.md) articles; updated [tool proposal](plans/midi-tool-interface.md) and [index](index.md). MIDI assignments are vendor documentation, not a hardware observation.

## [2026-09-28] query | Rekordbox MIDI connection investigation

Added [MIDI connection investigation plan](plans/rekordbox-midi-connection-investigation.md) and updated [index](index.md). The plan separates OS endpoint discovery, FLX4 capture, virtual-source mapping, message encoding, and Rekordbox MIDI output tests. No machine-specific observations have been made.

## [2026-09-29] ingest | System 1 inference and CLM

Archived two papers ([Talker–Reasoner](../raw/system-one/talker-reasoner-2410.08328.pdf), [DPT-Agent](../raw/system-one/dual-process-collaboration-2502.11882.pdf)) and the [CLM README](../raw/system-one/clm-repository-readme.md). Added [source notes](sources/talker-reasoner.md), [DPT-Agent notes](sources/dpt-agent.md), [CLM notes](sources/clm.md) and [inference comparison](system-one-inference.md); updated [index](index.md). CLM's linked publication is a blog, not a verified paper; no Jev latency tests have run.

## [2026-09-29] query | XDJ-AZ EQ recording experiment

Added [the standalone EQ recording plan](plans/xdj-az-eq-recording-experiment.md) and updated [the index](index.md). The plan includes a local MIDI port-discovery and raw capture app, audio recording, isolated knob identification, and explicit failure criteria. The [vendor MIDI list](https://downloads.support.alphatheta.com/software_info/all-in-one-dj-systems/XDJ-AZ/XDJ-AZ_MIDI_Message_List_E1.pdf) is a candidate reference; no XDJ-AZ hardware test has run.

## [2026-09-29] ingest | XDJ-AZ top-view image

Archived the [JPEG](../raw/xdj-az/xdj-az-cgi-top.jpg) and [provenance](../raw/xdj-az/SOURCES.md). Added [visual source note](sources/xdj-az-top-image.md) identifying the channel and master EQ controls; updated the [EQ experiment](plans/xdj-az-eq-recording-experiment.md) and [index](index.md). The rendering is not evidence of MIDI or audio behavior.

## [2026-09-29] query | XDJ-AZ board event inventory

Saved [deck and mixer board events](board-events.md) from the [top-view image](sources/xdj-az-top-image.md), including per-channel and shared mixer controls. Updated the [index](index.md). This is a visual inventory, not a verified event stream or MIDI mapping.

## [2026-09-29] ingest | Standalone hardware architecture and Rust deployment

Added [research links](../raw/standalone-hardware-architecture-2026-09-29.md), [source note](sources/standalone-hardware-architecture.md), and [deployment assessment](standalone-deployment.md). Updated the [index](index.md). The research distinguishes third-party firmware reports, service documentation, and vendor source distribution. No free-memory measurement, plugin support, or onboard Jev deployment is established.
