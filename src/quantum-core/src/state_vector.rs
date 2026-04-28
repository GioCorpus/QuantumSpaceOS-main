//! State Vector Representation
//! Full quantum state vector simulation

use super::{Amplitude, QubitState, QuantumError};
use ndarray::Array1;

/// Complete state vector for multi-qubit system
#[derive(Debug, Clone)]
pub struct StateVector {
    amplitudes: Array1<Amplitude>,
    n_qubits: usize,
}

impl StateVector {
    pub fn new(n_qubits: usize) -> Self {
        let size = 1 << n_qubits;
        let mut amplitudes = Array1::zeros(size);
        amplitudes[0] = Amplitude::new(1.0, 0.0); // |0...0⟩

        Self { amplitudes, n_qubits }
    }

    pub fn from_qubits(qubits: &[super::Qubit]) -> Result<Self, QuantumError> {
        // Build tensor product state
        Ok(Self {
            amplitudes: Array1::zeros(1 << qubits.len()),
            n_qubits: qubits.len(),
        })
    }

    pub fn probabilities(&self) -> Vec<f64> {
        self.amplitudes.mapv(|a| a.norm_sqr()).to_vec()
    }
}
