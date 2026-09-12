use std::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum SerializedAggregateStateError {
    #[error("aggregate state serialization failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("aggregate state decoding failed: {0}")]
    AggregateState(#[source] Box<dyn Error + Send + Sync>),
}
