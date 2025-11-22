use thiserror::Error;

#[derive(Debug, Error)]
pub enum AggregateIdOwnedError {
    #[error("aggregate id must be a valid uuid: {value}")]
    InvalidUuid {
        value: String,
        #[source]
        source: uuid::Error,
    },
}
