# Flight Simulator

## Overview

Orbital mechanics, thrust vectoring, atmospheric entry, and attitude control simulation module.

## Components

| File | Module |
|------|--------|
| `src/main.rs` | CLI entry point |


- `orbital_mechanics.rs` — Keplerian orbits, ground track
- `thrust_vectoring.rs` — Hohmann transfers, delta-V budgeting
- `atmospheric_entry.rs` — Reentry dynamics, aerodynamic heating
- `attitude_control.rs` — ACS, reaction wheels, thrusters

## Usage CLI

```bash
# Low Earth Orbit simulation
flight-sim --mode orbital --altitude 400

# Mars orbital insertion
flight-sim --mission mars-insertion --delta-v 2.5

# Atmospheric reentry
flight-sim --mode atmospheric-entry --velocity 7.8

# Interactive TUI
flight-sim --interactive
```

## Real-Time Telemetry

The flight-sim emits structured telemetry that can be consumed by the Python API:

```json
{
  "timestamp": "2026-04-28T08:00:00Z",
  "orbit": {
    "altitude_km": 400.2,
    "velocity_ms": 7670.5,
    "inclination_deg": 51.6
  },
  "attitude": {
    "roll_deg": 0.1,
    "pitch_deg": -0.2,
    "yaw_deg": 0.0
  }
}
```
