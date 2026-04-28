"""
Quantum Client — Interface to quantum-core and photonicq-bridge.
Provides high-level functions for quantum operations and telemetry.
"""

import asyncio
import json
from typing import Optional
import httpx

class QuantumClient:
    """Client for interacting with local quantum-core or remote quantum services"""

    def __init__(self, api_url: str = "http://localhost:8080"):
        self.api_url = api_url
        self.client = httpx.AsyncClient(timeout=10.0)

    async def close(self):
        await self.client.aclose()

    async def get_telemetry(self) -> dict:
        """Fetch full telemetry snapshot"""
        resp = await self.client.get(f"{self.api_url}/api/v1/telemetry")
        resp.raise_for_status()
        return resp.json()

    async def get_orbit(self) -> dict:
        resp = await self.client.get(f"{self.api_url}/api/v1/telemetry/orbit")
        resp.raise_for_status()
        return resp.json()

    async def get_quantum_state(self) -> dict:
        resp = await self.client.get(f"{self.api_url}/api/v1/quantum/state")
        resp.raise_for_status()
        return resp.json()

    async def run_circuit(self, gates: list[dict], shots: int = 1024) -> dict:
        """Submit quantum circuit for execution"""
        payload = {
            "qubits": list(range(len(gates))),  # placeholder
            "gates": gates,
            "shots": shots,
        }
        resp = await self.client.post(
            f"{self.api_url}/api/v1/quantum/run",
            json=payload
        )
        resp.raise_for_status()
        return resp.json()

    async def send_command(self, command: str, parameters: Optional[dict] = None) -> dict:
        """Send a command to the flight system"""
        payload = {
            "command": command,
            "parameters": parameters or {},
        }
        resp = await self.client.post(f"{self.api_url}/api/v1/command", json=payload)
        resp.raise_for_status()
        return resp.json()

# Synchronous wrapper for convenience
class SyncQuantumClient:
    """Blocking wrapper around QuantumClient"""

    def __init__(self, api_url: str = "http://localhost:8080"):
        self._async_client = QuantumClient(api_url)

    def get_telemetry(self):
        return asyncio.run(self._async_client.get_telemetry())

    def get_orbit(self):
        return asyncio.run(self._async_client.get_orbit())

    def run_circuit(self, gates, shots=1024):
        return asyncio.run(self._async_client.run_circuit(gates, shots))

    def send_command(self, command, parameters=None):
        return asyncio.run(self._async_client.send_command(command, parameters))
