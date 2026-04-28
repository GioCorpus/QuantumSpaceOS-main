# System Architecture

## Overview

QuantumSpaceOS is a hybrid quantum-photonic operating system designed for extreme space environments. This document details its layered architecture.

## Layered Architecture

```
 ┌─────────────────────────────────────────────────────────────┐
 │                    Application Layer                       │
 │  ┌───────────┐  ┌──────────┐  ┌─────────────┐             │
 │  │Flight Sim │  │Wayland GUI│  │ Telemetry API│             │
 │  └───────────┘  └──────────┘  └─────────────┘             │
 ├─────────────────────────────────────────────────────────────┤
 │                    Quantum Services Layer                  │
 │  ┌─────────────────┐      ┌───────────────────────┐        │
 │  │Quantum Core (Rust)│  ↔  │ PhotonicQ-Bridge     │        │
 │  │  • Qubits        │      │  • Optical encoding  │        │
 │  │  • Gates         │      │  • Entanglement      │        │
 │  │  • Algorithms    │      │  • Channel           │        │
 │  └─────────────────┘      └───────────────────────┘        │
 ├─────────────────────────────────────────────────────────────┤
 │                System Layer (Arch Linux)                   │
 │  • Linux kernel 6.x+          • systemd                    │
 │  • Wayland compositor         • PipeWire (optional)        │
 │  • Rust runtime               • Python runtime             │
 └─────────────────────────────────────────────────────────────┘
 │              Hardware Abstraction Layer                     │
 │  • x86_64 / ARM64             • Photonic transceivers      │
 │  • QPU interface (future)     • Radiation-hardened devices │
 └─────────────────────────────────────────────────────────────┘
```

## Component Interactions

### Flight Sim ↔ Quantum Core

Flight-sim uses quantum algorithms for trajectory optimization:

```rust
// Example: VQE for minimum-fuel transfer
let vqe = VQE::new(3);
let energy = vqe.minimize(&orbital_hamiltonian);
let dv = calculate_delta_v_from(energy);
thrust_controller.apply_dv(dv);
```

### PhotonicQ-Bridge ↔ Quantum Core

Bidirectional state transfer:

1. Quantum core creates `Qubit` state
2. Photonic encoder maps amplitudes to photon polarization/time-bin
3. Channel transmits photons with simulated loss and noise
4. Photonic decoder reconstructs amplitudes at receiver
5. Quantum core receives reconstructed `Qubit`

### Telemetry API ↔ All Components

All Rust binaries can emit structured JSON telemetry to the Python API via:

- Shared memory ring buffer (preferred, low-latency)
- Unix domain sockets
- HTTP (Wayland GUI and external clients)

## Build System

The ISO is built with `archiso`:

1. `scripts/build_iso.sh` → calls `mkarchiso`
2. `config/pacman.conf` sets package repositories
3. `config/mkinitcpio.conf` → builds initramfs with custom hooks
4. `config/grub.cfg` → boot menu configuration
5. `config/quantum.conf` → runtime system configuration

Result: a bootable ~2.5 GB ISO that runs entirely from RAM (toram).

## Security Model

- All quantum state data is held in memory only; never persisted to disk
- Telemetry API has no authentication by default (intended for isolated hardware)
- Photonic encryption uses one-time pads derived from quantum entropy
- System updates require signed packages

## Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Boot → shell | <15 s | from power-on to bash prompt |
| Memory footprint | <800 MB | with all services |
| Quantum simulation throughput | >1000 gate ops/ms | single-core |
| Photonic throughput | >10 Gbps simulated | channel |
| GUI refresh | 60 Hz capped | to save power |
| Power draw (idle) | <15 W | headless |

## Hardware Requirements

**Minimum:**
- x86_64 CPU (SSE4.2, 2+ cores)
- 4 GB RAM
- 20 GB storage (for ISO + overlay)
- UEFI boot

**Recommended:**
- x86_64 CPU (AVX2, 4+ cores)
- 8 GB RAM
- NVMe SSD
- Hardware KVM support

**Quantum Hardware (future):**
- Integration with QPU via PCIe or USB
- Photonic transceiver card (future)

---

See also:
- [QUANTUM_CORE.md](QUANTUM_CORE.md)
- [FLIGHT_SIM.md](FLIGHT_SIM.md)
- [TELEMETRY_API.md](TELEMETRY_API.md)
