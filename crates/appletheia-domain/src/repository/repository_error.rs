use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("aggregate not found")]
    AggregateNotFound,

    #[error("aggregate version not found")]
    AggregateVersionNotFound,
}
