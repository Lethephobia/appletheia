use thiserror::Error;

/// Represents errors returned while executing a pool token transfer.
#[derive(Debug, Error)]
pub enum PoolTokenTransferExecutorError {
    #[error("pool token transfer executor backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
