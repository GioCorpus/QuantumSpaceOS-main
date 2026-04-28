//! QuantumCore — Núcleo Cuántico de QuantumSpaceOS
//!
//! Proporciona:
//! - Simulación de qubits y circuitos cuánticos
//! - Puertas cuánticas básicas y compuestas
//! - Algoritmos cuánticos para navegación espacial
//! - Interfaz con Q# para simulación avanzada
//!
//! ## Ejemplo
//!
//! ```rust
//! use quantum_core::{Qubit, QubitState, apply_hadamard};
//!
//! let mut qubit = Qubit::zero();
//! apply_hadamard(&mut qubit);
//! let prob = qubit.measure();
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod qubit;
pub mod gates;
pub mod algorithms;
pub mod state_vector;
pub mod error;

pub use qubit::{Qubit, QubitState};
pub use gates::{Gate, single_q, multi_q};
pub use algorithms::{GroverSearch, QuantumFourierTransform, ShorAlgorithm};
pub use error::QuantumError;

/// Initialize the quantum core with Q# integration
pub fn init_qsharp() -> Result<(), QuantumError> {
    #[cfg(feature = "qsharp")]
    {
        // Load Q# runtime if feature enabled
        Ok(())
    }
    #[cfg(not(feature = "qsharp"))]
    {
        Err(QuantumError::FeatureNotAvailable(
            "Q# integration requires 'qsharp' feature".into()
        ))
    }
}

/// Run a quantum circuit and return measurement probabilities
pub fn run_circuit<F>(circuit: F, shots: usize) -> Result<Vec<f64>, QuantumError>
where
    F: FnOnce(&mut Qubit),
{
    let mut results = vec![0.0; 2];
    for _ in 0..shots {
        let mut qubit = Qubit::zero();
        circuit(&mut qubit);
        let outcome = qubit.measure();
        results[outcome as usize] += 1.0 / shots as f64;
    }
    Ok(results)
}

pub mod prelude {
    pub use crate::{Qubit, QubitState, Gate, QuantumError, run_circuit};
    pub use crate::gates::{H, X, Y, Z, CNOT, Toffoli};
    pub use crate::algorithms::{GroverSearch, QuantumFourierTransform};
}
