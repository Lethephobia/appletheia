use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventHistoryReaderError {
    #[error("aggregate not found")]
    AggregateNotFound,

    #[error("aggregate version not found")]
    AggregateVersionNotFound,
}
