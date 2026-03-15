use thiserror::Error;

/// Errors returned while constructing `Aes256GcmAuthTokenExchangeGrantCipher`.
#[derive(Debug, Error)]
pub enum Aes256GcmAuthTokenExchangeGrantCipherError {
    #[error("invalid AES-256-GCM key bytes")]
    InvalidKeyBytes(#[source] aes_gcm::aes::cipher::InvalidLength),
}
