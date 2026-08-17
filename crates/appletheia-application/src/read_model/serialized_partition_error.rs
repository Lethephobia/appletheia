use thiserror::Error;

/// Reports an invalid serialized read model partition.
#[derive(Debug, Error)]
pub enum SerializedPartitionError {
    #[error("read model partition must not be null")]
    NullPartition,
    #[error("failed to serialize read model fragment key")]
    SerializeKey(#[source] serde_json::Error),
    #[error("read model partition has an invalid shape")]
    InvalidShape,
    #[error("read model fragment partition type mismatch: expected {expected}, got {actual}")]
    FragmentMismatch { expected: String, actual: String },
    #[error("failed to deserialize read model fragment key")]
    DeserializeKey(#[source] serde_json::Error),
}
