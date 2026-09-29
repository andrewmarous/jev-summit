# DPT-Agent: real-time simultaneous human–AI collaboration

Source: [Zhang et al., *Leveraging Dual Process Theory in Language Agent Framework for Real-time Simultaneous Human-AI Collaboration* (2025), arXiv:2502.11882](https://arxiv.org/abs/2502.11882); [archived PDF](../../raw/system-one/dual-process-collaboration-2502.11882.pdf).

In the paper's architecture (Figure 1), System 1 is a **finite-state machine plus code-as-policy** that selects executable actions in the environment; it is not an LLM decoding a fresh action sequence on every tick. System 2 uses theory-of-mind modeling and asynchronous reflection to reason about a human collaborator and inform System 1. The evaluation concerns simultaneous collaboration in Overcooked, not audio or DJ hardware. Its measured performance does not establish Jev's control-loop latency.
