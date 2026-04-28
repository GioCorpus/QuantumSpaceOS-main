"""
FastAPI routes for QuantumSpaceOS telemetry and control API.
"""

from fastapi import FastAPI, HTTPException, BackgroundTasks
from fastapi.middleware.cors import CORSMiddleware
from typing import Optional
import random
from datetime import datetime
import asyncio

from .models import (
    TelemetryData, OrbitData, AttitudeData, QuantumState,
    CommandRequest, CommandResponse, SimulationMode,
    QuantumCircuitRequest, QuantumCircuitResult
)

app = FastAPI(
    title="QuantumSpaceOS Telemetry API",
    description="REST API for mission telemetry, quantum state, and flight control",
    version="0.1.0",
)

# CORS for local development
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# --- In-memory state (will be replaced by real telemetry source) ---
current_telemetry = TelemetryData(
    timestamp=datetime.utcnow(),
    mission_time_s=0.0,
    mode=SimulationMode.orbital,
    orbit=OrbitData(altitude_km=400.0, velocity_ms=7670.0, inclination_deg=51.6, period_s=5550.0),
    attitude=AttitudeData(roll_deg=0.0, pitch_deg=0.0, yaw_deg=0.0, angular_rate_dps=[0.0,0.0,0.0]),
    quantum=QuantumState(qubit_count=4, entanglement_fidelity=0.95, qber=0.01),
    health={"orbit": True, "attitude": True, "quantum": True, "comms": True}
)
command_queue: list[CommandRequest] = []

# --- Telemetry endpoints ---

@app.get("/api/v1/telemetry", response_model=TelemetryData)
async def get_telemetry():
    """Get current full telemetry snapshot"""
    return current_telemetry

@app.get("/api/v1/telemetry/orbit", response_model=OrbitData)
async def get_orbit():
    """Get orbital parameters only"""
    return current_telemetry.orbit

@app.get("/api/v1/telemetry/attitude", response_model=AttitudeData)
async def get_attitude():
    """Get attitude data only"""
    return current_telemetry.attitude

@app.get("/api/v1/telemetry/quantum", response_model=QuantumState)
async def get_quantum():
    """Get quantum processor state"""
    return current_telemetry.quantum

# --- Command & Control ---

@app.post("/api/v1/command", response_model=CommandResponse)
async def post_command(cmd: CommandRequest):
    """Submit a command to the flight system"""
    command_queue.append(cmd)

    # Simulate command processing
    result = process_command(cmd)

    return CommandResponse(
        command_id=f"cmd-{len(command_queue)}",
        status="accepted" if result.success else "failed",
        result=result.data,
        error=result.error,
    )

class CommandResult:
    def __init__(self, success: bool, data: Optional[dict]=None, error: Optional[str]=None):
        self.success = success
        self.data = data
        self.error = error

def process_command(cmd: CommandRequest) -> CommandResult:
    """Execute command on simulation state"""
    global current_telemetry

    try:
        match cmd.command:
            case "set_orbit_altitude":
                alt = cmd.parameters.get("altitude_km")
                if alt is not None:
                    current_telemetry.orbit.altitude_km = float(alt)
                    return CommandResult(True, {"new_altitude": alt})
                return CommandResult(False, error="Missing altitude_km parameter")

            case "set_mode":
                mode_str = cmd.parameters.get("mode")
                if mode_str:
                    current_telemetry.mode = SimulationMode(mode_str)
                    return CommandResult(True, {"mode": mode_str})
                return CommandResult(False, error="Missing mode parameter")

            case "fire_thrusters":
                dv = cmd.parameters.get("delta_v_ms", 0.0)
                # Simulate increasing velocity
                current_telemetry.orbit.velocity_ms += dv
                return CommandResult(True, {"new_velocity": current_telemetry.orbit.velocity_ms})

            case "reset":
                current_telemetry = TelemetryData(
                    timestamp=datetime.utcnow(),
                    mission_time_s=0.0,
                    mode=SimulationMode.orbital,
                    orbit=OrbitData(altitude_km=400.0, velocity_ms=7670.0, inclination_deg=51.6),
                    attitude=AttitudeData(roll_deg=0.0, pitch_deg=0.0, yaw_deg=0.0),
                    quantum=QuantumState(qubit_count=4, entanglement_fidelity=0.95, qber=0.01),
                    health={"orbit": True, "attitude": True, "quantum": True, "comms": True}
                )
                return CommandResult(True, {"status": "reset_complete"})

            case _:
                return CommandResult(False, error=f"Unknown command: {cmd.command}")
    except Exception as e:
        return CommandResult(False, error=str(e))

# --- Quantum Circuit API ---

@app.post("/api/v1/quantum/run", response_model=QuantumCircuitResult)
async def run_quantum_circuit(req: QuantumCircuitRequest):
    """Execute a quantum circuit (simulation)"""
    import time
    start = time.time()

    # Placeholder: would call quantum-core or Q#
    # For now return simulated result
    shots = req.shots
    # Simulate 50/50 outcome
    measurements = {"0": shots // 2, "1": shots // 2}
    probabilities = {"0": 0.5, "1": 0.5}

    elapsed_ms = (time.time() - start) * 1000

    return QuantumCircuitResult(
        measurements=measurements,
        probabilities=probabilities,
        execution_time_ms=elapsed_ms,
    )

@app.get("/api/v1/quantum/state")
async def get_quantum_state():
    """Get current quantum state of the processor"""
    return {
        "qubits": 4,
        "state_vector": [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        "fidelity": 0.95,
    }

# --- System ---

@app.get("/api/v1/system/health")
async def system_health():
    return {"status": "healthy", "uptime_s": 0, "services": {"telemetry": "up", "quantum": "up"}}

@app.get("/")
async def root():
    return {
        "name": "QuantumSpaceOS Telemetry API",
        "version": "0.1.0",
        "endpoints": {
            "telemetry": "/api/v1/telemetry",
            "commands": "/api/v1/command",
            "quantum": "/api/v1/quantum/run",
        }
    }
