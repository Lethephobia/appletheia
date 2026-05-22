use thiserror::Error;

/// Represents errors returned while creating an on-chain mint account.
#[derive(Debug, Error)]
pub enum MintAccountCreatorError {
    #[error("mint account creator backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
