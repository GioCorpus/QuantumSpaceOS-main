//! Attitude Control System (ACS)
//! Spacecraft orientation control using reaction wheels and thrusters

use nalgebra::{Vector3, UnitQuaternion};
use uom::si::f64::*;
use uom::si::angle::degree;
use uom::si::angular_velocity::degree_per_second;

/// Attitude state in quaternion form
#[derive(Debug, Clone)]
pub struct AttitudeState {
    /// Orientation quaternion (body relative to ECI)
    pub orientation: UnitQuaternion<f64>,
    /// Angular velocity (rad/s)
    pub angular_velocity: Vector3<f64>,
    /// Control torques (Nm)
    pub control_torques: Vector3<f64>,
}

impl AttitudeState {
    pub fn new() -> Self {
        Self {
            orientation: UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            angular_velocity: Vector3::zeros(),
            control_torques: Vector3::zeros(),
        }
    }

    /// Apply angular velocity delta
    pub fn integrate(&mut self, dt: f64) {
        // Simple Euler integration
        self.angular_velocity += self.control_torques * dt;

        // Update orientation
        let dq = UnitQuaternion::from_euler_angles(
            self.angular_velocity.x * dt,
            self.angular_velocity.y * dt,
            self.angular_velocity.z * dt,
        );
        self.orientation = dq * self.orientation;
    }

    /// Get Euler angles (roll, pitch, yaw) in degrees
    pub fn euler_angles(&self) -> (f64, f64, f64) {
        let euler = self.orientation.euler_angles();
        (euler.0.to_degrees(), euler.1.to_degrees(), euler.2.to_degrees())
    }
}

/// Reaction wheel controller
pub struct ReactionWheelController {
    /// Wheel inertias (kg·m²)
    wheel_inertias: Vector3<f64>,
    /// Maximum torque per wheel (Nm)
    max_torque: f64,
    /// Proportional gain
    kp: f64,
    /// Derivative gain
    kd: f64,
}

impl ReactionWheelController {
    pub fn new(wheel_inertias: Vector3<f64>) -> Self {
        Self {
            wheel_inertias,
            max_torque: 0.5,
            kp: 10.0,
            kd: 2.0,
        }
    }

    /// Compute control torques to achieve target attitude
    pub fn control_torque(
        &self,
        current: &AttitudeState,
        target: &AttitudeState,
    ) -> Vector3<f64> {
        // Compute error quaternion
        let error_q = target.orientation * current.orientation.inverse();
        let (roll, pitch, _yaw) = error_q.euler_angles();

        // PD control on Euler angles
        let angular_error = Vector3::new(roll, pitch, 0.0);
        let rate_error = target.angular_velocity - current.angular_velocity;

        let torque = self.kp * angular_error + self.kd * rate_error;

        // Clamp to max torque
        torque.mapv(|x| x.max(-self.max_torque).min(self.max_torque))
    }
}

/// Thruster-based attitude control
pub struct ThrusterACS {
    thruster_positions: Vec<Vector3<f64>>,
    thruster_directions: Vec<Vector3<f64>>,
    thrust_per_thruster: f64,
}

impl ThrusterACS {
    pub fn new() -> Self {
        Self {
            thruster_positions: vec![
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, -1.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(0.0, 0.0, -1.0),
            ],
            thruster_directions: vec![
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, -1.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(0.0, 0.0, -1.0),
            ],
            thrust_per_thruster: 1.0,
        }
    }

    /// Compute required thruster firings to achieve desired torque
    pub fn compute_impulses(&self, desired_torque: Vector3<f64>) -> Vec<f64> {
        // Simplified: solve linear system
        // T = Σ (r_i × F_i)
        // For our symmetric configuration, we approximate
        vec![
            desired_torque.x / 1.0,
            -desired_torque.x / 1.0,
            desired_torque.y / 1.0,
            -desired_torque.y / 1.0,
            desired_torque.z / 1.0,
            -desired_torque.z / 1.0,
        ]
        .into_iter()
        .map(|x| x.max(0.0).min(1.0))
        .collect()
    }
}

/// High-level Attitude Control System
pub struct AttitudeControl {
    state: AttitudeState,
    reaction_controller: ReactionWheelController,
    thruster_acs: ThrusterACS,
    mode: ControlMode,
}

#[derive(Debug, Clone, Copy)]
pub enum ControlMode {
    /// Reaction wheels only (fine control)
    Wheels,
    /// Thrusters only (coarse/emergency)
    Thrusters,
    /// Hybrid
    Combined,
}

impl AttitudeControl {
    pub fn new() -> Self {
        Self {
            state: AttitudeState::new(),
            reaction_controller: ReactionWheelController::new(Vector3::new(0.1, 0.1, 0.1)),
            thruster_acs: ThrusterACS::new(),
            mode: ControlMode::Wheels,
        }
    }

    /// Stabilize to nadir-pointing (Earth pointing)
    pub fn stabilize_nadir(&mut self, orbit: &mut super::orbital_mechanics::OrbitalState, dt: f64) {
        // Target: nadir direction (toward Earth center)
        let nadir = -orbit.position.normalize();
        let target_orientation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), nadir.angle(&Vector3::z_axis()));

        let target_state = AttitudeState {
            orientation: target_orientation,
            angular_velocity: Vector3::zeros(),
            control_torques: Vector3::zeros(),
        };

        self.stabilize(&target_state, dt);
    }

    /// Stabilize to a target AttitudeState
    pub fn stabilize(&mut self, target: &AttitudeState, dt: f64) {
        let torque = self.reaction_controller.control_torque(&self.state, target);
        self.state.control_torques = torque;

        self.state.integrate(dt);
    }

    pub fn state(&self) -> &AttitudeState {
        &self.state
    }
}

impl Default for AttitudeControl {
    fn default() -> Self {
        Self::new()
    }
}
