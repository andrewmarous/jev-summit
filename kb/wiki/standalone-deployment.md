# Low-memory standalone deployment

Source: [standalone hardware architecture research](sources/standalone-hardware-architecture.md). See [raw research links](../raw/standalone-hardware-architecture-2026-09-29.md).

## Hardware evidence

| Unit | Evidence | Limit |
| --- | --- | --- |
| XDJ-RX3 | Third-party firmware research reports i.MX 6Quad, 32-bit ARMv7-A, and Linux. | No free-memory or plugin-access measurement. |
| XDJ-XZ | A service manual identifies an i.MX6 main processor. | Exact variant and free memory remain unknown. |
| XDJ-RR | A service manual identifies an i.MX6 Dual processor. | Free memory remains unknown. |
| XDJ-AZ | The vendor distributes Linux-related open-source packages. | Exact processor and architecture remain unverified. |
| Original Denon PRIME 4 | Third-party firmware research reports armv7l Linux. | Do not generalize to other Engine OS units. |

The main CPU does not establish the architecture of every audio or control subsystem. None of these sources demonstrates a supported third-party plugin API.

## Design assessment

**Proposal:** Use Rust for a small board-control core. Rust supports 32-bit ARM/Linux targets, so the firmware's implementation language need not match the core's language. Cross-compilation alone does not establish binary compatibility with a device's libraries or operating system.

**Proposal:** Keep bounded queues and buffers, limit threads, and measure peak memory and allocation behavior on a specific target. Audio analysis and local model inference need separate memory budgets. Keep the board-control core separate from the listener and inference components described in [vision](vision.md) and [system model](system-model.md).

**Open:** For each target model and firmware version, establish the deployment path, ABI, available RAM, CPU headroom, and access to controls and audio. If onboard deployment is unavailable, the core can run beside the board. No onboard Jev deployment or memory benchmark has run.
