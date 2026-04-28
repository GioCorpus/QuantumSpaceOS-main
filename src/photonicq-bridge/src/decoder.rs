//! Photonic State Decoder
//! Converts photonic representations back to quantum states

use crate::{PhotonicQubit, error::BridgeError};
use num_complex::Complex;

pub struct Decoder {
    config: crate::ChannelConfig,
}

impl Decoder {
    pub fn new() -> Result<Self, BridgeError> {
        Ok(Self {
            config: crate::ChannelConfig::default(),
        })
    }

    /// Decode a photonic qubit back to a quantum state
    pub fn decode(&self, photonic: &PhotonicQubit) -> Result<super::super::Qubit, BridgeError> {
        // Reconstruct quantum state from photonic encoding
        let state = super::super::QubitState {
            alpha: photonic.alpha,
            beta: photonic.beta,
        };

        Ok(super::super::Qubit {
            state,
            coherence_time: std::time::Duration::from_micros(100),
        })
    }
}
