//! Photonic-Quantum Bridge Module
//!
//! Provides high-speed photonic communication interfaced with quantum
//! state processing. Enables qubit transmission over optical channels
//! with quantum error correction and entanglement preservation.
//!
//! ## Architecture
//!
//! ```text
//! Quantum States → Photonic Encoding → Fiber/Laser Link → Decoding → Qubits
//!      ↑                    ↑                    ↑              ↑
//!  quantum-core      modulators            photodetectors   quantum-core
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod encoder;
pub mod decoder;
pub mod channel;
pub mod entanglement;
pub mod error;

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// A photonic qubit representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotonicQubit {
    /// Wavelength in nanometers
    pub wavelength: f64,
    /// Polarization state (0-1)
    pub polarization: f64,
    /// Time bin encoding
    pub time_bin: Option<Duration>,
    /// Associated quantum state amplitudes
    pub alpha: Complex<f64>,
    pub beta: Complex<f64>,
    /// Error correction code
    pub ecc_code: Option<Vec<u8>>,
}

/// Photonic channel configuration
#[derive(Debug, Clone)]
pub struct ChannelConfig {
    /// Bandwidth in GHz
    pub bandwidth: f64,
    /// Attenuation in dB/km
    pub attenuation: f64,
    /// Quantum bit error rate threshold
    pub qber_threshold: f64,
    /// Use entanglement swapping
    pub entanglement_swapping: bool,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            bandwidth: 100.0,
            attenuation: 0.2,
            qber_threshold: 0.01,
            entanglement_swapping: true,
        }
    }
}

/// Main bridge interface
pub struct PhotonicBridge {
    config: ChannelConfig,
    encoder: encoder::Encoder,
    decoder: decoder::Decoder,
    channel: channel::PhotonicChannel,
}

impl PhotonicBridge {
    /// Create a new photonic bridge with default configuration
    pub fn new() -> Result<Self, error::BridgeError> {
        Ok(Self {
            config: ChannelConfig::default(),
            encoder: encoder::Encoder::new()?,
            decoder: decoder::Decoder::new()?,
            channel: channel::PhotonicChannel::new()?,
        })
    }

    /// Transmit a quantum state over the photonic channel
    pub async fn transmit(&mut self, qubit: &super::Qubit) -> Result<(), error::BridgeError> {
        let photonic = self.encoder.encode(qubit)?;
        self.channel.transmit(&photonic).await?;
        Ok(())
    }

    /// Receive a quantum state from the photonic channel
    pub async fn receive(&mut self) -> Result<super::Qubit, error::BridgeError> {
        let photonic = self.channel.receive().await?;
        self.decoder.decode(&photonic)
    }

    /// Establish entanglement between two nodes
    pub async fn entangle(
        &mut self,
        node_a: &str,
        node_b: &str,
    ) -> Result<entanglement::EntangledPair, error::BridgeError> {
        entanglement::create_entangled_pair(&self.channel, node_a, node_b).await
    }

    /// Get current channel quality metrics
    pub fn metrics(&self) -> ChannelMetrics {
        ChannelMetrics {
            signal_to_noise: self.channel.snr(),
            quantum_bit_error_rate: self.channel.qber(),
            bandwidth_utilization: self.channel.utilization(),
            entanglement_fidelity: self.channel.entanglement_fidelity(),
        }
    }
}

/// Channel performance metrics
#[derive(Debug, Clone)]
pub struct ChannelMetrics {
    pub signal_to_noise: f64,
    pub quantum_bit_error_rate: f64,
    pub bandwidth_utilization: f64,
    pub entanglement_fidelity: f64,
}

pub mod prelude {
    pub use crate::{ChannelConfig, ChannelMetrics, PhotonicBridge, PhotonicQubit};
}
