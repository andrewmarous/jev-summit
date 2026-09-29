# Agents Thinking Fast and Slow: A Talker-Reasoner Architecture

Source: [Christakopoulou, Mourad and Matarić (2024), arXiv:2410.08328](https://arxiv.org/abs/2410.08328); [archived PDF](../../raw/system-one/talker-reasoner-2410.08328.pdf).

The paper separates a fast, language-model **Talker** (System 1), which perceives and produces conversational responses, from a slower **Reasoner** (System 2), which plans, calls tools, updates beliefs and writes them to shared memory (§3.2). The Talker reads the most recent belief and can continue without waiting for a Reasoner update; it can also wait when a response requires fresh reasoning (§3.2). Thus the fast path may use stale beliefs. This is an agent architecture, not a new neural-network architecture or a proof of fixed real-time latency. Its example is a sleep-coaching conversation, not DJ control.
