//! Photonic Bridge Error Types

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BridgeError {
    #[error("Channel error: {0}")]
    ChannelError(String),

    #[error("Encoding failed: {0}")]
    EncodingError(String),

    #[error("Decoding failed: {0}")]
    DecodingError(String),

    #[error("Quantum bit error rate exceeded threshold")]
    QBERExceeded,

    #[error("Entanglement generation failed: {0}")]
    EntanglementError(String),
}
