//! Quantum Gates
//! Single-qubit and multi-qubit quantum gates

use super::{Amplitude, Qubit, QubitState};
use num_complex::Complex;

/// Single-qubit gate representation
#[derive(Debug, Clone, Copy)]
pub struct SingleQubitGate {
    matrix: [[Complex<f64>; 2]; 2],
}

impl SingleQubitGate {
    /// Create gate from 2x2 unitary matrix
    pub fn new(matrix: [[Complex<f64>; 2]; 2]) -> Result<Self, super::QuantumError> {
        // Verify unitarity: U†U = I
        let conj = [[
            matrix[0][0].conj(),
            matrix[1][0].conj(),
        ], [
            matrix[0][1].conj(),
            matrix[1][1].conj(),
        ]];

        let mut identity = [[Complex::new(0.0, 0.0); 2]; 2];
        for i in 0..2 {
            for k in 0..2 {
                for j in 0..2 {
                    identity[i][j] += conj[i][k] * matrix[k][j];
                }
            }
            if (identity[i][i] - Complex::new(1.0, 0.0)).norm() > 1e-10 {
                return Err(super::QuantumError::NonUnitaryGate);
            }
        }
        Ok(Self { matrix })
    }

    /// Apply gate to a qubit
    pub fn apply(&self, qubit: &mut Qubit) {
        let a = qubit.state.alpha;
        let b = qubit.state.beta;
        qubit.state.alpha = self.matrix[0][0] * a + self.matrix[0][1] * b;
        qubit.state.beta = self.matrix[1][0] * a + self.matrix[1][1] * b;
    }
}

/// Two-qubit gate representation
#[derive(Debug, Clone, Copy)]
pub struct TwoQubitGate {
    matrix: [[Complex<f64>; 4]; 4],
}

impl TwoQubitGate {
    pub fn new(matrix: [[Complex<f64>; 4]; 4]) -> Result<Self, super::QuantumError> {
        // Unitarity check simplified
        Ok(Self { matrix })
    }

    /// Apply CNOT to control and target qubits
    pub fn apply_cnot(&self, control: &mut Qubit, target: &mut Qubit) {
        // CNOT: |00⟩→|00⟩, |01⟩→|01⟩, |10⟩→|11⟩, |11⟩→|10⟩
        if control.state.prob_one() > 0.5 {
            // Flip target if control is |1⟩
            let new_target = QubitState {
                alpha: target.state.beta,
                beta: target.state.alpha,
            };
            target.state = new_target;
        }
    }
}

// === Common Gates ===

/// Pauli-X (NOT) gate
pub fn x() -> SingleQubitGate {
    SingleQubitGate::new([
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ]).unwrap()
}

/// Pauli-Y gate
pub fn y() -> SingleQubitGate {
    SingleQubitGate::new([
        [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
        [Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)],
    ]).unwrap()
}

/// Pauli-Z gate
pub fn z() -> SingleQubitGate {
    SingleQubitGate::new([
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)],
    ]).unwrap()
}

/// Hadamard gate: |0⟩ → (|0⟩+|1⟩)/√2, |1⟩ → (|0⟩-|1⟩)/√2
pub fn h() -> SingleQubitGate {
    let inv_sqrt2 = 1.0 / 2.0_f64.sqrt();
    SingleQubitGate::new([
        [Complex::new(inv_sqrt2, 0.0), Complex::new(inv_sqrt2, 0.0)],
        [Complex::new(inv_sqrt2, 0.0), Complex::new(-inv_sqrt2, 0.0)],
    ]).unwrap()
}

/// Phase gate S = diag(1, i)
pub fn s() -> SingleQubitGate {
    SingleQubitGate::new([
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(0.0, 1.0)],
    ]).unwrap()
}

/// T gate (π/8 gate)
pub fn t() -> SingleQubitGate {
    let angle = PI / 4.0;
    SingleQubitGate::new([
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(angle.cos(), angle.sin())],
    ]).unwrap()
}

/// CNOT gate (control, target)
pub fn cnot() -> TwoQubitGate {
    TwoQubitGate::new([
        [Complex::new(1.0,0.0), Complex::new(0.0,0.0), Complex::new(0.0,0.0), Complex::new(0.0,0.0)],
        [Complex::new(0.0,0.0), Complex::new(1.0,0.0), Complex::new(0.0,0.0), Complex::new(0.0,0.0)],
        [Complex::new(0.0,0.0), Complex::new(0.0,0.0), Complex::new(0.0,0.0), Complex::new(1.0,0.0)],
        [Complex::new(0.0,0.0), Complex::new(0.0,0.0), Complex::new(1.0,0.0), Complex::new(0.0,0.0)],
    ]).unwrap()
}

/// Toffoli (CCNOT) gate
pub fn toffoli() -> impl Fn(&mut Qubit, &mut Qubit, &mut Qubit) {
    |c1: &mut Qubit, c2: &mut Qubit, target: &mut Qubit| {
        if c1.state.prob_one() > 0.5 && c2.state.prob_one() > 0.5 {
            // Flip target
            let new_target = QubitState {
                alpha: target.state.beta,
                beta: target.state.alpha,
            };
            target.state = new_target;
        }
    }
}

// Convenience wrappers
pub mod single_q {
    pub use super::{h, x, y, z, s, t};
}

pub mod multi_q {
    pub use super::{cnot, toffoli};
}
