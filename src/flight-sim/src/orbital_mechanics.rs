//! Orbital Mechanics Module
//! Two-body problem, keplerian elements, orbital propagation

use nalgebra::Vector3;
use uom::si::f64::*;
use uom::si::length::{kilometer, meter};
use uom::si::time::{second, hour};
use uom::si::velocity::{kilometer_per_second, meter_per_second};
use uom::si::angle::radian;

/// Orbital state in ECI (Earth-Centered Inertial) frame
#[derive(Debug, Clone)]
pub struct OrbitalState {
    /// Position vector (m)
    pub position: Vector3<f64>,
    /// Velocity vector (m/s)
    pub velocity: Vector3<f64>,
    /// Semi-major axis (m)
    pub semi_major_axis: f64,
    /// Eccentricity
    pub eccentricity: f64,
    /// Inclination (rad)
    pub inclination: f64,
    /// RAAN (rad)
    pub raan: f64,
    /// Argument of perigee (rad)
    pub arg_perigee: f64,
    /// True anomaly (rad)
    pub true_anomaly: f64,
    /// Current altitude above surface (m)
    pub altitude: f64,
    /// Orbital period (s)
    pub period: f64,
}

impl OrbitalState {
    /// Create new Earth LEO state
    pub fn new() -> Self {
        // Standard LEO: 400km circular, 51.6° inclination (ISS-like)
        let altitude = 400_000.0; // m
        let earth_radius = 6_371_000.0;
        let r = earth_radius + altitude;
        let v = (3.986004418e14 / r).sqrt(); // Circular orbit velocity

        Self {
            position: Vector3::new(r, 0.0, 0.0),
            velocity: Vector3::new(0.0, v, 0.0),
            semi_major_axis: r,
            eccentricity: 0.0,
            inclination: 51.6f64.to_radians(),
            raan: 0.0,
            arg_perigee: 0.0,
            true_anomaly: 0.0,
            altitude,
            period: 2.0 * std::f64::consts::PI * (r.powi(3) / 3.986004418e14).sqrt(),
        }
    }

    /// Mars orbital insertion state
    pub fn mars_insertion() -> Self {
        // Placeholder: Mars orbit at 300km altitude
        let altitude = 300_000.0;
        let mars_radius = 3_389_500.0;
        let r = mars_radius + altitude;
        let v = (3.971e13 / r).sqrt(); // Mars mu ≈ 3.971×10^13 m^3/s^2

        Self {
            position: Vector3::new(r, 0.0, 0.0),
            velocity: Vector3::new(0.0, v, 0.0),
            semi_major_axis: r,
            eccentricity: 0.0,
            inclination: 25.0f64.to_radians(),
            raan: 0.0,
            arg_perigee: 0.0,
            true_anomaly: 0.0,
            altitude,
            period: 2.0 * std::f64::consts::PI * (r.powi(3) / 3.971e13).sqrt(),
        }
    }

    /// Lunar transfer orbit (TLI)
    pub fn lunar_transfer() -> Self {
        // Approximate translunar injection
        let perigee_alt = 185_000.0; // km
        let apogee_alt = 384_400_000.0; // Moon distance
        let earth_radius = 6_371_000.0;
        let r_per = earth_radius + perigee_alt;
        let r_ap = earth_radius + apogee_alt;
        let a = (r_per + r_ap) / 2.0;
        let e = (r_ap - r_per) / (r_ap + r_per);

        let v_per = (3.986004418e14 * (2.0/r_per - 1.0/a)).sqrt();

        Self {
            position: Vector3::new(r_per, 0.0, 0.0),
            velocity: Vector3::new(0.0, v_per, 0.0),
            semi_major_axis: a,
            eccentricity: e,
            inclination: 28.5f64.to_radians(), // Typical TLI inclination
            raan: 0.0,
            arg_perigee: 0.0,
            true_anomaly: 0.0,
            altitude: perigee_alt,
            period: 2.0 * std::f64::consts::PI * (a.powi(3) / 3.986004418e14).sqrt(),
        }
    }

    /// Propagate orbit using simple Keplerian mechanics
    pub fn propagate(&mut self, dt: f64) {
        // Advance mean anomaly (simplified circular)
        let mean_motion = 2.0 * std::f64::consts::PI / self.period;
        let delta_m = mean_motion * dt;
        self.true_anomaly += delta_m;

        // Update position and velocity (circular approx)
        let r = self.semi_major_axis;
        let v = (3.986004418e14 / r).sqrt();

        self.position = Vector3::new(
            r * self.true_anomaly.cos(),
            r * self.true_anomaly.sin(),
            0.0,
        );
        self.velocity = Vector3::new(
            -v * self.true_anomaly.sin(),
            v * self.true_anomaly.cos(),
            0.0,
        );

        self.altitude = r - 6_371_000.0; // Earth radius
    }

    /// Get current ground track (latitude, longitude)
    pub fn ground_track(&self) -> (f64, f64) {
        let lat = self.position.y.atan2(self.position.x) * self.inclination.sin();
        let lon = self.position.z.atan2(self.position.x); // Simplified
        (lat.to_degrees(), lon.to_degrees())
    }
}
