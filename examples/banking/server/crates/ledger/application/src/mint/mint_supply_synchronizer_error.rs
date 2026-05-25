use thiserror::Error;

/// Represents errors returned while synchronizing on-chain mint supply.
#[derive(Debug, Error)]
pub enum MintSupplySynchronizerError {
    #[error("mint supply synchronizer backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
