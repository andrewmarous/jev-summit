# System model

Sources: [project conversation](sources/project-conversation.md) and [Karpathy wiki pattern](sources/karpathy-llm-wiki.md). The architecture below is a proposal, not external documentation for Rekordbox.

The proposed path is: Jev control request → local board adapter → Rekordbox. The observation path is: Rekordbox state and audio → local capture → future listener → Jev. This is a proposed design, not a tested integration.

## Distinct capabilities

- **Write:** Send a control action to Rekordbox.
- **Read:** Observe Rekordbox's actual state after any human or software action. A locally remembered value is not readback.
- **Hear:** Capture master and private cue audio independently. A master-only feed does not replace headphones.
- **Time:** Relate each observation and action to playback time. Timing precision is not established.

**Hypothesis:** Virtual MIDI through macOS IAC can provide some write controls. Mapping coverage and feedback remain unknown. Audio routing depends on the user's setup.

## Open questions

- What is the user's Rekordbox version, mode, plan, controller, and audio interface?
- Can a virtual MIDI device select headphone cue without changing the master?
- Can software capture headphone cue and master as separate streams on that setup?
- Is actual deck and mixer state available through MIDI feedback or another supported interface?
- Can a human override software actions without leaving the adapter's state stale?

The [v0.1 probe](plans/rekordbox-probe-v0.1.md) tests these questions before implementation expands.
