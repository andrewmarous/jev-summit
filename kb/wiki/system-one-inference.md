# System 1 inference for Jev

“System 1” names a **fast decision role**, not a standard neural architecture. Three relevant implementations differ:

| Approach | Fast-path inference | Slow-path relationship |
| --- | --- | --- |
| [Talker–Reasoner](sources/talker-reasoner.md) | Language model generates a conversational response; token decoding still takes time. | Reasoner updates shared beliefs asynchronously; Talker may wait for fresh beliefs. |
| [DPT-Agent](sources/dpt-agent.md) | FSM and code-as-policy execute bounded actions without a new LLM call per tick. | Asynchronous reflection updates the policy's guidance. |
| [CLM-v0.1](sources/clm.md) | Embed text state, compare with candidate-action embeddings, rank/select; no action-token decoding. | No required System 2 component in the released scoring API; a planner could supply candidates or policy updates. |

**Project hypothesis:** keep safety limits and time-critical board actions on a bounded fast path; update goals, candidate actions and beliefs on a slower path. This is not a verified design. CLM-v0.1 is text-only, while Jev's proposed audio listener is still unselected ([vision](vision.md)). Any audio-driven use needs a measured audio-to-text/state pipeline or a future validated multimodal model.

Measure end-to-end time from audio/control observation timestamp to applied MIDI command, including capture window, feature extraction, state encoding, decision, transport, and actuation. Record p50/p95/p99, stale-state age, missed deadlines and unsafe actions under load. Choose a target latency budget only after measuring the musical control task and hardware path. None of the cited papers establishes that budget for Jev.
