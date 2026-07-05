use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoolTokenTransferExecutorError {
    #[error("pool token transfer was rejected")]
    Rejected,

    #[error("backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
