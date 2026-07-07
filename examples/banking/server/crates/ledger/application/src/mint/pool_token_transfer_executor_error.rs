use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoolTokenTransferExecutorError {
    #[error("backend failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}
