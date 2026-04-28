//! Thrust Vectoring Module
//! Rocket engine control and delta-V calculation

use nalgebra::Vector3;
use uom::si::f64::*;
use uom::si::force::newton;
use uom::si::mass::kilogram;
use uom::si::time::second;

/// Thrust controller for powered flight phases
#[derive(Debug)]
pub struct ThrustController {
    /// Thrust magnitude (N)
    current_thrust: f64,
    /// Thrust direction (unit vector)
    direction: Vector3<f64>,
    /// Mass flow rate (kg/s)
    mass_flow: f64,
    /// Specific impulse (s)
    isp: f64,
    /// Thrust active flag
    active: bool,
    /// Fuel remaining (kg)
    fuel_remaining: f64,
}

impl ThrustController {
    pub fn new() -> Self {
        Self {
            current_thrust: 0.0,
            direction: Vector3::new(1.0, 0.0, 0.0),
            mass_flow: 0.1,
            isp: 300.0, // Typical chemical engine ISP
            active: false,
            fuel_remaining: 1000.0,
        }
    }

    /// Set thrust magnitude
    pub fn set_thrust(&mut self, magnitude: f64) {
        self.current_thrust = magnitude;
    }

    /// Set thrust direction (unit vector)
    pub fn set_direction(&mut self, dir: Vector3<f64>) {
        let len = dir.norm();
        if len > 0.0 {
            self.direction = dir / len;
        }
    }

    /// Activate thrust (consumes fuel)
    pub fn activate(&mut self) {
        if self.fuel_remaining > 0.0 {
            self.active = true;
        }
    }

    /// Deactivate thrust
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Apply thrust to orbital state
    pub fn apply(&mut self, orbit: &mut super::orbital_mechanics::OrbitalState, dt: f64) {
        if !self.active || self.fuel_remaining <= 0.0 {
            self.current_thrust = 0.0;
            return;
        }

        // Calculate acceleration: a = F/m
        // Simplified: constant mass approximation
        let acceleration = self.current_thrust / 1000.0; // Assume 1000kg spacecraft

        // Apply delta-v
        let dv = acceleration * dt;
        orbit.velocity += self.direction * dv;

        // Consume fuel
        let fuel_used = self.mass_flow * dt;
        self.fuel_remaining -= fuel_used;

        if self.fuel_remaining <= 0.0 {
            self.deactivate();
        }
    }

    /// Calculate total delta-V budget from Tsiolkovsky equation
    pub fn delta_v_budget(&self) -> f64 {
        self.isp * 9.81 * (self.fuel_remaining / 1000.0).ln() // g₀ ≈ 9.81 m/s²
    }

    /// Perform a Hohmann transfer maneuver
    pub fn hohmann_transfer(
        &mut self,
        initial: &super::orbital_mechanics::OrbitalState,
        target_altitude: f64,
    ) -> (f64, f64) {
        // Compute delta-V for two-impulse transfer
        let r1 = initial.semi_major_axis;
        let r2 = target_altitude + 6_371_000.0; // Earth radius + altitude

        let mu = 3.986004418e14; // Earth gravitational parameter

        // First impulse: raise apogee
        let v1 = (mu / r1).sqrt();
        let v_perigee_transfer = mu * (2.0/r1 - 1.0/((r1+r2)/2.0)).sqrt();
        let dv1 = v_perigee_transfer - v1;

        // Second impulse: circularize at target
        let v2_transfer = mu * (2.0/r2 - 1.0/((r1+r2)/2.0)).sqrt();
        let v2 = (mu / r2).sqrt();
        let dv2 = v2 - v2_transfer;

        (dv1, dv2)
    }
}
