//! Quantum Core Error Types

use thiserror::Error;

#[derive(Debug, Error)]
pub enum QuantumError {
    #[error("Invalid quantum state: {0}")]
    InvalidState(String),

    #[error("Gate operation failed: {0}")]
    GateError(String),

    #[error("Non-unitary gate applied")]
    NonUnitaryGate,

    #[error("Qubit decohered")]
    DecoherenceError,

    #[error("Measurement collapsed unexpectedly")]
    MeasurementError,

    #[error("Feature not available: {0}")]
    FeatureNotAvailable(String),

    #[error("Simulation overflow")]
    OverflowError,

    #[error("State vector size mismatch")]
    StateVectorError,
}
