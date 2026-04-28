//! Atmospheric Entry Module
//! Reentry dynamics, heating, and deceleration

use nalgebra::Vector3;
use uom::si::f64::*;
use uom::si::temperature::kelvin;
use uom::si::pressure::pascal;

/// Atmospheric model (US Standard 1976, simplified)
struct Atmosphere {
    /// Scale height (m)
    scale_height: f64,
    /// Sea level density (kg/m³)
    sea_level_density: f64,
    /// Sea level temperature (K)
    sea_level_temp: f64,
}

impl Atmosphere {
    fn new() -> Self {
        Self {
            scale_height: 8500.0, // Approx scale height
            sea_level_density: 1.225,
            sea_level_temp: 288.15,
        }
    }

    fn density(&self, altitude: f64) -> f64 {
        // Exponential atmosphere model
        self.sea_level_density * (-altitude / self.scale_height).exp()
    }

    fn temperature(&self, altitude: f64) -> f64 {
        // Simplified linear lapse rate for troposphere
        let lapse = 6.5; // K/km
        self.sea_level_temp - lapse * altitude / 1000.0
    }
}

/// Reentry vehicle state
#[derive(Debug, Clone)]
pub struct ReentryState {
    /// Position (ECI)
    pub position: Vector3<f64>,
    /// Velocity (m/s)
    pub velocity: Vector3<f64>,
    /// Mass (kg)
    pub mass: f64,
    /// Ballistic coefficient (kg/m²)
    pub ballistic_coeff: f64,
    /// Heat shield status (0-1)
    pub heat_shield_integrity: f64,
    /// Peak heating rate (W/m²)
    pub peak_heating: f64,
}

impl ReentryState {
    /// Initialize reentry from orbit
    pub fn from_orbit(altitude: f64, velocity: f64) -> Self {
        Self {
            position: Vector3::new(altitude + 6_371_000.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, velocity, 0.0),
            mass: 5000.0, // Typical capsule mass
            ballistic_coeff: 100.0, // kg/m²
            heat_shield_integrity: 1.0,
            peak_heating: 0.0,
        }
    }
}

/// Atmospheric entry simulator
pub struct AtmosphericEntrySim {
    atmosphere: Atmosphere,
    state: ReentryState,
    g_earth: f64,
}

impl AtmosphericEntrySim {
    pub fn new(altitude: f64, velocity: f64) -> Self {
        Self {
            atmosphere: Atmosphere::new(),
            state: ReentryState::from_orbit(altitude, velocity),
            g_earth: 9.81,
        }
    }

    /// Propagate one time step
    pub fn step(&mut self, dt: f64) {
        let alt = self.altitude();
        let rho = self.atmosphere.density(alt);
        let vel = self.velocity_magnitude();

        // Drag force: F_d = ½ ρ v² C_d A
        // Using ballistic coefficient β = m / (C_d A) → F_d = ½ ρ v² m / β
        let drag_acc = 0.5 * rho * vel.powi(2) / self.state.ballistic_coeff;

        // Deceleration opposite to velocity
        let mut drag_vec = -self.state.velocity.normalize() * drag_acc;
        drag_vec.z += self.g_earth; // Add gravity

        // Update velocity and position
        self.state.velocity += drag_vec * dt;
        self.state.position += self.state.velocity * dt;

        // Consume heat shield proportional to heating rate
        let heating = 1.83e-4 * rho.powf(0.5) * vel.powf(3.0); // Simplified Chapman heating
        self.state.peak_heating = self.state.peak_heating.max(heating);
        self.state.heat_shield_integrity -= heating * dt * 1e-8;

        // Decay mass if ablating
        if self.state.heat_shield_integrity < 0.8 {
            self.state.mass -= 0.1 * dt; // kg/s ablation
        }
    }

    /// Current altitude above surface
    fn altitude(&self) -> f64 {
        let r_earth = 6_371_000.0;
        self.state.position.norm() - r_earth
    }

    /// Velocity magnitude
    fn velocity_magnitude(&self) -> f64 {
        self.state.velocity.norm()
    }

    /// Check for safe landing
    pub fn landed(&self) -> bool {
        self.altitude() < 100.0 && self.velocity_magnitude() < 10.0 // <100m, <10m/s
    }

    pub fn state(&self) -> &ReentryState {
        &self.state
    }
}
