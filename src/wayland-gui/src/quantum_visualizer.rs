//! Quantum Visualization Module
//! Visualizes qubit states, entanglement, and quantum circuits

use photonicq_bridge::prelude::*;
use quantum_core::prelude::*;
use nalgebra::Vector2;

/// Bloch sphere visualizer for single qubit
pub struct BlochSphere {
    radius: f64,
    center: Vector2<f64>,
    state: Option<QubitState>,
}

impl BlochSphere {
    pub fn new(center: Vector2<f64>, radius: f64) -> Self {
        Self {
            radius,
            center,
            state: None,
        }
    }

    /// Update displayed state
    pub fn set_state(&mut self, state: QubitState) {
        self.state = Some(state);
    }

    /// Render Bloch sphere to frame buffer
    pub fn render(&self, frame: &mut [u8], width: u32, height: u32) -> Result<(), QuantumVisError> {
        let stride = width as usize * 4;

        // Draw wireframe sphere (circle projection)
        self.draw_circle(frame, width, height, self.radius as i32, [0x33, 0x33, 0x33, 0xFF])?;

        // Draw axes
        let axis_len = self.radius as i32;
        let cx = self.center.x as i32;
        let cy = self.center.y as i32;

        // Z-axis (vertical)
        self.draw_line(frame, cx, cy, cx, cy - axis_len, [0xFF, 0x00, 0x00, 0xFF])?; // Z
        self.draw_line(frame, cx, cy, cx + axis_len, cy, [0x00, 0xFF, 0x00, 0xFF])?; // X
        self.draw_line(frame, cx, cy, cx, cy + axis_len, [0x00, 0x00, 0xFF, 0xFF])?; // -Z? Actually we want Y
        // Fix axes properly: Z up, X right, Y into screen (elliptical)
        // For 2D projection: show X and Z

        // Draw state vector tip
        if let Some(state) = &self.state {
            let theta = 2.0 * (state.beta.norm()).acos(); // polar angle
            let phi = state.beta.arg() - state.alpha.arg(); // azimuth

            let x = self.radius * theta.sin() * phi.cos();
            let z = self.radius * theta.cos();

            // Project to 2D: X horizontal, Z vertical (up)
            let tip_x = cx + x as i32;
            let tip_y = cy - z as i32; // Screen Y down

            // Draw state vector
            self.draw_line(frame, cx, cy, tip_x, tip_y, [0xFF, 0xFF, 0x00, 0xFF])?;
            // Draw tip dot
            self.draw_circle(frame, tip_x, tip_y, 4, [0xFF, 0xFF, 0x00, 0xFF])?;
        }

        Ok(())
    }

    fn draw_circle(&self, frame: &mut [u8], w: u32, h: u32, r: i32, color: [u8; 4]) -> Result<(), QuantumVisError> {
        let stride = w as usize * 4;
        let cx = self.center.x as i32;
        let cy = self.center.y as i32;

        for dy in -r..=r {
            let dx = ((r.pow(2) - dy.pow(2)) as f64).sqrt() as i32;
            self.set_pixel(frame, w, h, cx - dx, cy + dy, color);
            self.set_pixel(frame, w, h, cx + dx, cy + dy, color);
        }
        Ok(())
    }

    fn draw_line(&self, frame: &mut [u8], x0: i32, y0: i32, x1: i32, y1: i32, color: [u8; 4]) -> Result<(), QuantumVisError> {
        // Bresenham line
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut x = x0;
        let mut y = y0;
        let mut err = dx - dy;

        let w = self.center.x as u32 * 2; // approximate
        let h = self.center.y as u32 * 2;

        loop {
            self.set_pixel(frame, w, h, x, y, color);
            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 > -dy { err -= dy; x += sx; }
            if e2 < dx { err += dx; y += sy; }
        }
        Ok(())
    }

    fn set_pixel(&self, frame: &mut [u8], width: u32, height: u32, x: i32, y: i32, color: [u8; 4]) {
        if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
            let idx = (y as usize * (width as usize * 4)) + (x as usize * 4);
            if idx + 3 < frame.len() {
                frame[idx..idx+4].copy_from_slice(&color);
            }
        }
    }
}

/// Multi-qubit state visualizer (entanglement diagram)
pub struct QuantumVisualizer {
    qubits: Vec<QubitState>,
    entangled_pairs: Vec<(usize, usize)>,
    bloch_spheres: Vec<BlochSphere>,
    width: u32,
    height: u32,
}

impl QuantumVisualizer {
    pub fn new() -> Self {
        Self {
            qubits: vec![QubitState::zero(); 4],
            entangled_pairs: vec![(0, 1), (2, 3)],
            bloch_spheres: Vec::new(),
            width: 1280,
            height: 720,
        }
    }

    pub fn update(&mut self) -> Result<(), QuantumVisError> {
        // Update from quantum-core state
        // For now, simulate some dynamics
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Randomly rotate some qubits
        for i in 0..self.qubits.len() {
            let angle = rng.gen_range(0.0..0.1);
            self.qubits[i].rotate_phase(angle);
        }

        Ok(())
    }

    pub fn render(&self, frame: &mut [u8]) -> Result<(), QuantumVisError> {
        // Layout: 2x2 grid of Bloch spheres
        let spacing = 50;
        let radius = 80;
        let positions = [
            (200, 150),
            (600, 150),
            (200, 450),
            (600, 450),
        ];

        for (i, &(px, py)) in positions.iter().enumerate() {
            if i < self.qubits.len() {
                let mut sphere = BlochSphere::new(
                    Vector2::new(px as f64, py as f64),
                    radius as f64,
                );
                sphere.set_state(self.qubits[i].clone());
                sphere.render(frame, self.width, self.height)?;

                // Qubit label
                self.draw_label(frame, px, py + (radius as i32) + 20, &format!("q[{}]", i))?;
            }
        }

        // Draw entanglement lines
        self.draw_entanglement_lines(frame, &positions)?;

        Ok(())
    }

    fn draw_entanglement_lines(&self, frame: &mut [u8], positions: &[(i32, i32)]) -> Result<(), QuantumVisError> {
        for &(a, b) in &self.entangled_pairs {
            if a < positions.len() && b < positions.len() {
                let (x0, y0) = positions[a];
                let (x1, y1) = positions[b];
                // Draw animated dashed line (solid for now)
                self.draw_line(frame, x0, y0, x1, y1, [0xFF, 0x00, 0xFF, 0x80])?;
            }
        }
        Ok(())
    }

    fn draw_line(&self, frame: &mut [u8], x0: i32, y0: i32, x1: i32, y1: i32, color: [u8; 4]) -> Result<(), QuantumVisError> {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut x = x0;
        let mut y = y0;
        let mut err = dx - dy;
        let stride = 4;

        loop {
            let idx = (y as usize * (self.width as usize * stride)) + (x as usize * 4);
            if idx + 3 < frame.len() {
                frame[idx..idx+4].copy_from_slice(&color);
            }
            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 > -dy { err -= dy; x += sx; }
            if e2 < dx { err += dx; y += sy; }
        }
        Ok(())
    }

    fn draw_label(&self, frame: &mut [u8], x: i32, y: i32, text: &str) -> Result<(), QuantumVisError> {
        // Placeholder: single colored pixel per char
        let mut cx = x;
        for _ in text.chars() {
            let idx = (y as usize * (self.width as usize * 4)) + (cx as usize * 4);
            if idx + 3 < frame.len() {
                frame[idx..idx+4].copy_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);
            }
            cx += 6;
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum QuantumVisError {
    #[error("Render error")]
    RenderError,
}
