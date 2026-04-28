"""
Flight simulator unit tests.

Run with: python -m pytest tests/flight_tests.py
"""

import pytest
from src.flight_sim.orbital_mechanics import OrbitalState

def test_leo_initialization():
    orbit = OrbitalState.new()
    # Altitude ~400km
    assert 390_000 < orbit.altitude < 410_000
    # Circular orbit -> e ≈ 0
    assert orbit.eccentricity < 0.01

def test_propagation_circular():
    orbit = OrbitalState.new()
    initial_alt = orbit.altitude
    orbit.propagate(60.0)  # 1 minute
    # Altitude should stay roughly constant for circular
    assert abs(orbit.altitude - initial_alt) < 100.0  # within 100m

def test_mars_insertion():
    mars = OrbitalState.mars_insertion()
    # Mars radius ~3390 km
    expected_alt = 300_000.0
    assert abs(mars.altitude - expected_alt) < 1000.0

def test_ground_track_returns_latlon():
    orbit = OrbitalState.new()
    lat, lon = orbit.ground_track()
    assert -90.0 <= lat <= 90.0
    assert -180.0 <= lon <= 180.0
