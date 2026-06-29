use thiserror::Error;

/// Represents errors returned while validating a token account owner address.
#[derive(Debug, Error)]
pub enum TokenAccountOwnerAddressValidatorError {
    #[error("token account owner address is invalid")]
    InvalidAddress,

    #[error("token account owner address validator backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
