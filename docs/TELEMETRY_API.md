# Telemetry API Documentation

## Overview

The Telemetry API (`src/api/`) is a FastAPI application that exposes:
- Real-time flight data
- Quantum state information
- Command & Control (C2) endpoint

It runs on port 8080 by default.

## Quick Start

```bash
# Activate Python virtualenv
source .venv/bin/activate

# Install dependencies
pip install -r src/api/requirements.txt

# Start the server
python src/api/telemetry_api.py

# Or with uvicorn directly:
uvicorn src.api.telemetry_api:app --host 0.0.0.0 --port 8080 --reload
```

## REST Endpoints

### Telemetry

| Method | Endpoint | Returns | Description |
|--------|----------|---------|-------------|
| `GET` | `/api/v1/telemetry` | `TelemetryData` | Full snapshot |
| `GET` | `/api/v1/telemetry/orbit` | `OrbitData` | Orbital only |
| `GET` | `/api/v1/telemetry/attitude` | `AttitudeData` | Attitude only |
| `GET` | `/api/v1/telemetry/quantum` | `QuantumState` | Quantum core state |

Example:

```bash
curl http://localhost:8080/api/v1/telemetry | python -m json.tool
```

Response:

```json
{
  "timestamp": "2026-04-28T08:00:00Z",
  "mission_time_s": 1234.5,
  "mode": "orbital",
  "orbit": {
    "altitude_km": 400.2,
    "velocity_ms": 7670.1,
    "inclination_deg": 51.6,
    "period_s": 5550.0
  },
  "attitude": {
    "roll_deg": 0.01,
    "pitch_deg": -0.02,
    "yaw_deg": 0.0,
    "angular_rate_dps": [0.0,0.0,0.0]
  },
  "quantum": {
    "qubit_count": 4,
    "entanglement_fidelity": 0.95,
    "qber": 0.01
  },
  "health": {
    "orbit": true,
    "attitude": true,
    "quantum": true,
    "comms": true
  }
}
```

### Command & Control

`POST /api/v1/command` — send a command to the flight system:

```json
{
  "command": "set_orbit_altitude",
  "parameters": { "altitude_km": 500.0 }
}
```

Supported commands:

| Command | Parameters | Effect |
|---------|------------|--------|
| `set_orbit_altitude` | `{"altitude_km": 450}` | Set target altitude |
| `set_mode` | `{"mode": "mars_insertion"}` | Switch simulation mode |
| `fire_thrusters` | `{"delta_v_ms": 100}` | Instant Δv (debug) |
| `reset` | `{}` | Reset to initial state |

Response:

```json
{
  "command_id": "cmd-1",
  "status": "accepted",
  "result": { "new_altitude": 500.0 },
  "error": null
}
```

### Quantum Execution

`POST /api/v1/quantum/run` — execute a circuit:

```json
{
  "qubits": [0, 1],
  "gates": [
    { "type": "h", "target": 0 },
    { "type": "cnot", "control": 0, "target": 1 }
  ],
  "shots": 1024
}
```

Response:

```json
{
  "measurements": { "00": 512, "11": 512 },
  "probabilities": { "0": 0.5, "1": 0.5 },
  "execution_time_ms": 2.34
}
```

`GET /api/v1/quantum/state` — get current state vector (simulated):

```json
{
  "qubits": 4,
  "state_vector": [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  "fidelity": 0.95
}
```

### System Health

`GET /api/v1/system/health` — service health check:

```json
{
  "status": "healthy",
  "uptime_s": 3600,
  "services": { "telemetry": "up", "quantum": "up" }
}
```

`GET /` — API root, lists endpoints.

## Python Client

`QuantumClient` (async) and `SyncQuantumClient` wrapper:

```python
from src.api.quantum_client import SyncQuantumClient

client = SyncQuantumClient("http://localhost:8080")
telemetry = client.get_telemetry()
orbit = client.get_orbit()
result = client.run_circuit(gates=[...], shots=2048)
client.send_command("set_mode", {"mode": "mars_insertion"})
```

## Data Models

All models use Pydantic v2 for validation:

- `TelemetryData` — top-level snapshot
- `OrbitData`, `AttitudeData`, `QuantumState` — subspaces
- `CommandRequest`, `CommandResponse`
- `QuantumCircuitRequest`, `QuantumCircuitResult`

Fields include units and range validation (e.g., `altitude_km ≥ 0`).

## WebSocket Streaming (Planned)

Future versions will support `WS /api/v1/telemetry/stream` for low-latency updates.

---

See also: `ARCHITECTURE.md`
