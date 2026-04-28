"""
Telemetry and quantum data models for QuantumSpaceOS API.
Uses Pydantic for validation and serialization.
"""

from datetime import datetime
from typing import Optional
from pydantic import BaseModel, Field
from enum import Enum

class SimulationMode(str, Enum):
    orbital = "orbital"
    mars_insertion = "mars_insertion"
    atmospheric_entry = "atmospheric_entry"
    lunar = "lunar"

class OrbitData(BaseModel):
    """Orbital state data"""
    altitude_km: float = Field(..., ge=0, description="Altitude above surface in km")
    velocity_ms: float = Field(..., ge=0, description="Orbital velocity in m/s")
    inclination_deg: float = Field(..., ge=-90, le=90, description="Inclination in degrees")
    period_s: Optional[float] = Field(None, ge=0, description="Orbital period in seconds")
    raan_deg: Optional[float] = Field(None, description="Right ascension of ascending node")

class AttitudeData(BaseModel):
    """Attitude Control System state"""
    roll_deg: float = Field(..., description="Roll angle in degrees")
    pitch_deg: float = Field(..., description="Pitch angle in degrees")
    yaw_deg: float = Field(..., description="Yaw angle in degrees")
    angular_rate_dps: list[float] = Field(default_factory=lambda: [0.0, 0.0, 0.0], description="Angular rates [roll, pitch, yaw] in deg/s")

class QuantumState(BaseModel):
    """Quantum processor state"""
    qubit_count: int = Field(..., ge=0, description="Number of active qubits")
    entanglement_fidelity: float = Field(..., ge=0.0, le=1.0, description="Entanglement fidelity (0-1)")
    qber: float = Field(..., ge=0.0, le=1.0, description="Quantum Bit Error Rate")
    circuit_depth: Optional[int] = Field(None, ge=0, description="Current circuit depth")

class TelemetryData(BaseModel):
    """Complete telemetry snapshot"""
    timestamp: datetime = Field(default_factory=datetime.utcnow, description="UTC timestamp")
    mission_time_s: float = Field(..., ge=0, description="Mission elapsed time in seconds")
    mode: SimulationMode = Field(..., description="Current simulation mode")
    orbit: OrbitData
    attitude: AttitudeData
    quantum: QuantumState
    health: dict[str, bool] = Field(default_factory=dict, description="Subsystem health flags")

class CommandRequest(BaseModel):
    """Command from ground control"""
    command: str = Field(..., description="Command identifier")
    parameters: dict = Field(default_factory=dict, description="Command parameters")
    timestamp: datetime = Field(default_factory=datetime.utcnow)

class CommandResponse(BaseModel):
    """Acknowledgment of command execution"""
    command_id: str
    status: str
    result: Optional[dict] = None
    error: Optional[str] = None

class QuantumCircuitRequest(BaseModel):
    """Submit a quantum circuit for execution"""
    qubits: list[int]
    gates: list[dict]
    shots: int = Field(1024, ge=1, le=1000000)

class QuantumCircuitResult(BaseModel):
    """Result of quantum circuit execution"""
    measurements: dict[str, int]
    probabilities: dict[str, float]
    execution_time_ms: float
