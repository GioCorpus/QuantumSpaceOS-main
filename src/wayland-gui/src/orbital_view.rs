//! Orbital View Module
//! 3D visualization of spacecraft orbit around planetary bodies

use nalgebra::{Vector3, Unit, UnitQuaternion, Quaternion};
use bytemuck::{Pod, Zeroable};
use std::f64::consts::PI;

/// 3D camera for orbital view
#[derive(Debug, Clone)]
pub struct Camera {
    /// Eye position in ECI coordinates (m)
    pub eye: Vector3<f64>,
    /// Look-at target
    pub target: Vector3<f64>,
    /// Up vector
    pub up: Vector3<f64>,
    /// Field of view (radians)
    pub fov: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: Vector3::new(0.0, 10_000_000.0, 10_000_000.0),
            target: Vector3::zeros(),
            up: Vector3::y(),
            fov: PI / 4.0,
        }
    }
}

/// Orbital renderer state
pub struct OrbitalView {
    width: u32,
    height: u32,
    camera: Camera,
    planet_radius: f64,
    orbit_trail: Vec<Vector3<f64>>,
    max_trail: usize,
    time: f64,
}

impl OrbitalView {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            camera: Camera::default(),
            planet_radius: 6_371_000.0, // Earth
            orbit_trail: Vec::new(),
            max_trail: 500,
            time: 0.0,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Update orbit position from flight-sim
    pub fn update(&mut self, position: Vector3<f64>, velocity: Vector3<f64>) -> Result<(), OrbitalError> {
        // Update camera to follow (track from behind)
        let look_dir = velocity.normalize();
        let offset = look_dir * 1_000_000.0; // 1000km behind
        self.camera.eye = position - offset;
        self.camera.target = position;

        // Record trail point
        self.orbit_trail.push(position);
        if self.orbit_trail.len() > self.max_trail {
            self.orbit_trail.remove(0);
        }

        self.time += 0.016; // Approx 60fps

        Ok(())
    }

    /// Render orbital view to RGBA pixel buffer
    pub fn render(&self, frame: &mut [u8]) -> Result<(), OrbitalError> {
        // Simplified software renderer: draw planet circle, orbit trail, and spacecraft
        let stride = self.width as usize * 4;
        let center_x = self.width / 2;
        let center_y = self.height / 2;
        let scale = 0.0001; // m → pixel

        // Draw planet (circle)
        let planet_radius_px = (self.planet_radius * scale) as i32;
        self.draw_circle(frame, center_x, center_y, planet_radius_px, [0x1E, 0x90, 0xFF, 0xFF])?; // Earth blue

        // Draw orbit trail
        for (i, pos) in self.orbit_trail.iter().enumerate() {
            let x = center_x + (pos.x * scale) as i32;
            let y = center_y + (pos.y * scale) as i32; // simplified 2D projection
            let alpha = (i as f64 / self.orbit_trail.len() as f64 * 255.0) as u8;
            self.draw_pixel(frame, x, y, [0x00, 0xFF, 0x00, alpha]);
        }

        // Draw spacecraft at current position
        if let Some(current) = self.orbit_trail.last() {
            let sx = center_x + (current.x * scale) as i32;
            let sy = center_y + (current.y * scale) as i32;
            self.draw_circle(frame, sx, sy, 4, [0xFF, 0xFF, 0x00, 0xFF])?;
        }

        Ok(())
    }

    /// Draw a filled circle (Bresenham-like)
    fn draw_circle(&self, frame: &mut [u8], cx: i32, cy: i32, r: i32, color: [u8; 4]) -> Result<(), OrbitalError> {
        for dy in -r..=r {
            let dx = ((r.pow(2) - dy.pow(2)) as f64).sqrt() as i32;
            self.draw_hline(frame, cx - dx, cx + dx, cy + dy, color);
        }
        Ok(())
    }

    fn draw_hline(&self, frame: &mut [u8], x0: i32, x1: i32, y: i32, color: [u8; 4]) {
        let stride = self.width as usize * 4;
        for x in x0..=x1 {
            if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                let idx = (y as usize * stride) + (x as usize * 4);
                if idx + 3 < frame.len() {
                    frame[idx..idx+4].copy_from_slice(&color);
                }
            }
        }
    }

    fn draw_pixel(&self, frame: &mut [u8], x: i32, y: i32, color: [u8; 4]) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let stride = self.width as usize * 4;
            let idx = (y as usize * stride) + (x as usize * 4);
            if idx + 3 < frame.len() {
                frame[idx..idx+4].copy_from_slice(&color);
            }
        }
    }
}

/// Orbital view error type
#[derive(Debug, thiserror::Error)]
pub enum OrbitalError {
    #[error("Render error: {0}")]
    RenderError(String),
    #[error("Camera error: {0}")]
    CameraError(String),
}
