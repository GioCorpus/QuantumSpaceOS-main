"""
QuantumSpaceOS Telemetry API — Main Entry Point

Runs the FastAPI server that provides telemetry, command, and quantum execution endpoints.

Usage:
    python3 telemetry_api.py

Or with uvicorn:
    uvicorn telemetry_api:app --host 0.0.0.0 --port 8080

Endpoints:
    GET  /api/v1/telemetry          Full telemetry
    GET  /api/v1/telemetry/orbit    Orbital parameters
    POST /api/v1/command            Send flight command
    POST /api/v1/quantum/run        Execute quantum circuit
"""

import uvicorn
from fastapi import FastAPI
from . import routes

app = routes.app

def main():
    """Run the API server"""
    uvicorn.run(
        "src.api.telemetry_api:app",
        host="0.0.0.0",
        port=8080,
        reload=True,
        log_level="info",
    )

if __name__ == "__main__":
    main()
