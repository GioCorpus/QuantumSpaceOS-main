//! Qubit and State Representation
//! Fundamental quantum state container and operations

use num_complex::Complex;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Complex amplitude representation
pub type Amplitude = Complex<f64>;

/// Qubit state |ψ⟩ = α|0⟩ + β|1⟩
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QubitState {
    /// Amplitude for |0⟩
    pub alpha: Amplitude,
    /// Amplitude for |1⟩
    pub beta: Amplitude,
}

impl QubitState {
    /// Create a new qubit state from amplitudes
    pub fn new(alpha: Amplitude, beta: Amplitude) -> Result<Self, super::QuantumError> {
        let norm = alpha.norm_sqr() + beta.norm_sqr();
        if (norm - 1.0).abs() > 1e-10 {
            return Err(super::QuantumError::InvalidState(format!(
                "State not normalized: norm={}"
            )));
        }
        Ok(Self { alpha, beta })
    }

    /// Zero state |0⟩
    pub fn zero() -> Self {
        Self {
            alpha: Complex::new(1.0, 0.0),
            beta: Complex::new(0.0, 0.0),
        }
    }

    /// One state |1⟩
    pub fn one() -> Self {
        Self {
            alpha: Complex::new(0.0, 0.0),
            beta: Complex::new(1.0, 0.0),
        }
    }

    /// Plus state |+⟩ = (|0⟩ + |1⟩)/√2
    pub fn plus() -> Self {
        let norm = 1.0 / 2.0_f64.sqrt();
        Self {
            alpha: Complex::new(norm, 0.0),
            beta: Complex::new(norm, 0.0),
        }
    }

    /// Compute probability of measuring |1⟩
    pub fn prob_one(&self) -> f64 {
        self.beta.norm_sqr()
    }

    /// Compute probability of measuring |0⟩
    pub fn prob_zero(&self) -> f64 {
        self.alpha.norm_sqr()
    }

    /// Apply a phase rotation
    pub fn rotate_phase(&mut self, angle: f64) {
        let phase = Complex::new(angle.cos(), angle.sin());
        self.alpha *= phase;
    }
}

/// A qubit with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qubit {
    /// Quantum state
    pub state: QubitState,
    /// Coherence time remaining (simulated)
    pub coherence_time: std::time::Duration,
    /// Unique identifier
    pub id: String,
}

impl Qubit {
    /// Create a new |0⟩ qubit
    pub fn zero() -> Self {
        Self {
            state: QubitState::zero(),
            coherence_time: std::time::Duration::from_micros(1000),
            id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// Create a new |1⟩ qubit
    pub fn one() -> Self {
        Self {
            state: QubitState::one(),
            coherence_time: std::time::Duration::from_micros(1000),
            id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// Measure the qubit (collapses state)
    pub fn measure(&mut self) -> u8 {
        let p = self.state.prob_one();
        let outcome: u8 = if rand::thread_rng().gen::<f64>() < p { 1 } else { 0 };

        // Collapse state
        if outcome == 0 {
            self.state = QubitState::zero();
        } else {
            self.state = QubitState::one();
        }

        outcome
    }

    /// Measure without collapsing (weak measurement simulation)
    pub fn measure_weak(&self) -> f64 {
        self.state.prob_one()
    }

    /// Update coherence (call periodically)
    pub fn tick_coherence(&mut self, dt: std::time::Duration) {
        self.coherence_time = self.coherence_time.saturating_sub(dt);
    }

    /// Check if qubit has decohered
    pub fn is_decohered(&self) -> bool {
        self.coherence_time.is_zero()
    }
}

impl Default for Qubit {
    fn default() -> Self {
        Self::zero()
    }
}
