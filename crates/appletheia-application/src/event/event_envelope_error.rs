use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventEnvelopeError {
    #[error("event name mismatch: expected {expected}, got {actual}")]
    EventNameMismatch { expected: String, actual: String },

    #[error("aggregate type mismatch: expected {expected}, got {actual}")]
    AggregateTypeMismatch {
        expected: &'static str,
        actual: String,
    },

    #[error("aggregate id error")]
    AggregateId(#[source] Box<dyn Error + Send + Sync>),

    #[error("event payload error")]
    EventPayload(#[source] Box<dyn Error + Send + Sync>),
}
