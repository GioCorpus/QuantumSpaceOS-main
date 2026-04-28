# Wayland GUI

## Overview

Minimal Wayland client for QuantumSpaceOS mission control. Renders:
- Orbital view: 2D/3D projection of spacecraft trajectory
- Telemetry overlay: Real-time flight data
- Quantum visualizer: Bloch sphere rendering of qubit states

## Dependencies

- `winit` with Wayland backend
- `pixels` for software pixel buffer (no GPU driver required)
- `smithay-client-toolkit` for advanced Wayland protocols

## Usage

```bash
# Build
cargo build --release

# Run under Wayland compositor (Sway, River, Weston)
./target/release/quantumspace-gui --telemetry-url http://localhost:8080

# Fullscreen
./target/release/quantumspace-gui --fullscreen
```

## Architecture

```
main.rs         → Event loop, window, pixels init
orbital_view.rs → Orbital trajectory renderer
telemetry.rs    → HUD overlay for flight data
quantum_visualizer.rs → Bloch sphere display
```

## Design Goals

- **Low memory**: <50 MB RSS
- **Low CPU**: <10% single core at 60 Hz
- **No GPU required**: Software rendering via `pixels`
- **Fast startup**: <1s to visible window

## Extensibility

Future: replace `pixels` with `wgpu` for GPU acceleration; add `smithay` for custom Wayland protocols.
