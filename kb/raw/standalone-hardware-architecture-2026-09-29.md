# Standalone DJ hardware architecture: research links

Recorded 2026-09-29 from web research for the Rust and low-memory deployment question. These links are evidence of different quality. No device was opened or tested for this research. Processor details do not establish free RAM, a plugin API, or access to internal controls and audio.

- **XDJ-RX3:** [rx3-toolkit references](https://github.com/Tratosca/rx3-toolkit/blob/main/REFERENCES.md) report an NXP/Freescale i.MX 6Quad (32-bit ARMv7-A), Linux 3.0.101, U-Boot, BusyBox, and a C++ player application. This is third-party firmware research, not a vendor specification.
- **XDJ-XZ:** [service manual RRV4698](https://audiocircuit.dk/downloads/pioneer/Pioneer-XDJXZ-mp-sm1.pdf) identifies an i.MX6 main application processor. The exact variant and available memory are not established here.
- **XDJ-RR:** [service manual copy](https://www.scribd.com/document/434363880/XDJRR-ServMan1) identifies part MCIMX6D6AVT10AD, an i.MX6 Dual application processor. The copy is hosted by a third party. Memory available to third-party code is unknown.
- **XDJ-AZ:** [AlphaTheta GPL/LGPL source distribution](https://www.pioneerdj.com/fr/support/open-source-code-distribution/gnu-open-source-license/) lists Linux-related open-source packages. This does not establish the AZ's exact SoC, instruction-set variant, RAM budget, or ability to install third-party software.
- **Original Denon PRIME 4:** [firmware research README](https://github.com/icedream/denon-prime4/blob/icedream-experiments/README.md) reports armv7l Linux with a Buildroot root filesystem and a Qt Engine application. This is third-party research, not proof about other Engine OS devices or the PRIME 4+.

The main application processor does not describe every processor in a unit. Audio and control functions can use separate hardware. These sources do not prove that any of these boards supports a Jev plugin.
