//! Telemetry View Module
//! Displays real-time flight data and system health

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Telemetry data structure (mirrors API model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryData {
    pub timestamp: DateTime<Utc>,
    pub orbit: OrbitData,
    pub attitude: AttitudeData,
    pub quantum: QuantumData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitData {
    pub altitude_km: f64,
    pub velocity_ms: f64,
    pub inclination_deg: f64,
    pub period_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttitudeData {
    pub roll_deg: f64,
    pub pitch_deg: f64,
    pub yaw_deg: f64,
    pub angular_rate_dps: [f64; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumData {
    pub qubit_count: usize,
    pub entanglement_fidelity: f64,
    pub qber: f64, // Quantum Bit Error Rate
}

/// Telemetry UI component
pub struct TelemetryView {
    data: TelemetryData,
    scroll_offset: usize,
    font_size: u32,
}

impl TelemetryView {
    pub fn new() -> Self {
        Self {
            data: TelemetryData {
                timestamp: Utc::now(),
                orbit: OrbitData {
                    altitude_km: 400.0,
                    velocity_ms: 7670.0,
                    inclination_deg: 51.6,
                    period_s: 5550.0,
                },
                attitude: AttitudeData {
                    roll_deg: 0.0,
                    pitch_deg: 0.0,
                    yaw_deg: 0.0,
                    angular_rate_dps: [0.0, 0.0, 0.0],
                },
                quantum: QuantumData {
                    qubit_count: 4,
                    entanglement_fidelity: 0.95,
                    qber: 0.01,
                },
            },
            scroll_offset: 0,
            font_size: 12,
        }
    }

    /// Update telemetry (fetch from API or local source)
    pub fn update(&mut self) -> Result<(), TelemetryError> {
        // TODO: fetch from local telemetry_api or direct flight-sim IPC
        // For now, simulate slight variations
        use rand::Rng;
        let mut rng = rand::thread_rng();
        self.data.orbit.altitude_km += rng.gen_range(-0.1..0.1);
        self.data.attitude.roll_deg += rng.gen_range(-0.01..0.01);
        self.data.attitude.pitch_deg += rng.gen_range(-0.01..0.01);
        self.data.timestamp = Utc::now();

        Ok(())
    }

    /// Render telemetry overlay onto frame buffer
    pub fn render(&self, frame: &mut [u8], width: u32, height: u32) -> Result<(), TelemetryError> {
        let font_size = self.font_size as i32;
        let panel_x = (width as i32) - 300; // Right panel
        let start_y = 20;

        // Draw semi-transparent panel background
        self.draw_rect(frame, panel_x - 10, start_y - 10, 290, 200, [0x00, 0x00, 0x00, 0x80])?;

        // Draw title
        self.draw_text(frame, "TELEMETRY", panel_x, start_y, [0x00, 0xFF, 0xFF, 0xFF])?;

        let mut y = start_y + font_size + 10;

        // Orbit block
        self.draw_text(frame, "Orbit", panel_x, y, [0xFF, 0x00, 0x00, 0xFF])?;
        y += font_size + 5;
        self.draw_text(frame, &format!("Alt: {:.1} km", self.data.orbit.altitude_km), panel_x, y, [0xFF, 0xFF, 0xFF, 0xFF])?;
        y += font_size;
        self.draw_text(frame, &format!("Vel: {:.1} m/s", self.data.orbit.velocity_ms), panel_x, y, [0xFF, 0xFF, 0xFF, 0xFF])?;
        y += font_size;
        self.draw_text(frame, &format!("Inc: {:.1}°", self.data.orbit.inclination_deg), panel_x, y, [0xFF, 0xFF, 0xFF, 0xFF])?;
        y += font_size + 10;

        // Attitude block
        self.draw_text(frame, "Attitude", panel_x, y, [0x00, 0xFF, 0x00, 0xFF])?;
        y += font_size + 5;
        self.draw_text(frame, &format!("R: {:.3}°", self.data.attitude.roll_deg), panel_x, y, [0xFF, 0xFF, 0xFF, 0xFF])?;
        y += font_size;
        self.draw_text(frame, &format!("P: {:.3}°", self.data.attitude.pitch_deg), panel_x, y, [0xFF, 0xFF, 0xFF, 0xFF])?;
        y += font_size;
        self.draw_text(frame, &format!("Y: {:.3}°", self.data.attitude.yaw_deg), panel_x, y, [0xFF, 0xFF, 0xFF, 0xFF])?;
        y += font_size + 10;

        // Quantum block
        self.draw_text(frame, "Quantum", panel_x, y, [0xFF, 0x00, 0xFF, 0xFF])?;
        y += font_size + 5;
        self.draw_text(frame, &format!("Qubits: {}", self.data.quantum.qubit_count), panel_x, y, [0xFF, 0xFF, 0xFF, 0xFF])?;
        y += font_size;
        self.draw_text(frame, &format!("Fidelity: {:.2}%", self.data.quantum.entanglement_fidelity * 100.0), panel_x, y, [0xFF, 0xFF, 0xFF, 0xFF])?;
        y += font_size;
        self.draw_text(frame, &format!("QBER: {:.2}%", self.data.quantum.qber * 100.0), panel_x, y, [0xFF, 0xFF, 0xFF, 0xFF])?;

        Ok(())
    }

    /// Simple rectangle drawer
    fn draw_rect(&self, frame: &mut [u8], x: i32, y: i32, w: i32, h: i32, color: [u8; 4]) -> Result<(), TelemetryError> {
        let stride = 4; // RGBA
        for dy in 0..h {
            let row = y + dy;
            if row < 0 { continue; }
            for dx in 0..w {
                let col = x + dx;
                if col < 0 { continue; }
                let idx = (row as usize * stride * (self.width() as usize)) + (col as usize * 4);
                if idx + 3 < frame.len() {
                    frame[idx..idx+4].copy_from_slice(&color);
                }
            }
        }
        Ok(())
    }

    /// Simple text drawer (placeholder: draws rectangles as glyphs)
    fn draw_text(&self, frame: &mut [u8], text: &str, x: i32, y: i32, color: [u8; 4]) -> Result<(), TelemetryError> {
        // Placeholder: draw each character as a 8x12 box
        let char_w = 8;
        let char_h = 12;
        let mut cx = x;
        for ch in text.chars() {
            // Simple bounding box per character
            self.draw_rect(frame, cx, y, char_w, char_h, color)?;
            cx += char_w + 2;
        }
        Ok(())
    }

    fn width(&self) -> u32 {
        // Would come from parent window
        1280
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TelemetryError {
    #[error("Render error: {0}")]
    RenderError(String),
    #[error("Fetch error: {0}")]
    FetchError(String),
}
