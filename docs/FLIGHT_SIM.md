# Flight Simulator Documentation

## Overview

`flight-sim` (`src/flight-sim`) is the orbital and atmospheric simulation engine. It computes spacecraft motion under Keplerian mechanics, thrust, drag, and integrates ACS.

## Modules

### `orbital_mechanics.rs`

```rust
use flight_sim::orbital_mechanics::OrbitalState;

let mut orbit = OrbitalState::new(); // 400 km LEO
orbit.propagate(60.0); // advance 60 seconds
println!("Altitude: {} km", orbit.altitude / 1000.0);
```

**`OrbitalState`** fields:
- `position`, `velocity`: ECI vectors (m, m/s)
- `semi_major_axis`, `eccentricity`: Keplerian elements
- `inclination`, `raan`, `arg_perigee`, `true_anomaly`: orbital angles (rad)
- `altitude`: current altitude above reference ellipsoid
- `period`: orbital period (s)

**Methods**:
- `propagate(dt)`: simple time step integration
- `ground_track()` → `(lat, lon)` in degrees

### `thrust_vectoring.rs`

`ThrustController` manages engine thrust and computes Δv:

```rust
let mut thrust = ThrustController::new();
thrust.set_thrust(5000.0); // Newtons
thrust.set_direction([1.0, 0.0, 0.0].into());
thrust.activate();
thrust.apply(&mut orbit, 1.0); // 1 second burn
```

**Hohmann transfer**:

```rust
let (dv1, dv2) = thrust.hohmann_transfer(&initial_orbit, target_alt_km);
```

### `atmospheric_entry.rs`

`AtmosphericEntrySim` models reentry aerodynamics, heating, and ablation:

```rust
let mut reentry = AtmosphericEntrySim::new(altitude_km * 1000.0, velocity_km_s * 1000.0);
loop {
    reentry.step(0.1); // 0.1 s time step
    if reentry.landed() { break; }
}
```

Access state: `reentry.state()` → `ReentryState`

### `attitude_control.rs`

Two ACS modalities:

- **ReactionWheelController** — PD control for fine pointing
- **ThrusterACS** — Pulse-width modulated thrusters for larger torques

High-level: `AttitudeControl::stabilize_nadir(&mut orbit, dt)` automatically points nadir.

## CLI Usage

```bash
# Orbital simulation
flight-sim --mode orbital --altitude 400

# Mars insertion
flight-sim --mission mars-insertion --delta-v 2.5

# Atmospheric entry at 7.8 km/s
flight-sim --mode atmospheric-entry --velocity 7.8

# Interactive TUI (placeholder)
flight-sim --interactive
```

### JSON Output Formatting

When used as a library, flight-sim can serialize state via `serde`:

```rust
let json = serde_json::to_string(&orbit).unwrap();
```

See `telemetry_api` for canonical JSON schema.

## Integration

Flight-sim emits telemetry consumed by:
- `src/wayland-gui` — direct in-process or shared memory
- `src/api/telemetry_api.py` — HTTP polling
- External ground stations via custom protocol

---

See also: `ARCHITECTURE.md`
