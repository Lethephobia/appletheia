use thiserror::Error;

/// Represents errors returned while updating on-chain mint account metadata.
#[derive(Debug, Error)]
pub enum MintAccountMetadataUpdaterError {
    #[error("mint account metadata updater backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
