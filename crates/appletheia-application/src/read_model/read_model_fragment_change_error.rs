use thiserror::Error;

use super::{SerializedPartitionError, SerializedReadModelFragmentError};

/// Reports serialization or type errors in a physical fragment change.
#[derive(Debug, Error)]
pub enum ReadModelFragmentChangeError {
    #[error("failed to serialize a read model fragment")]
    SerializeFragment(#[source] serde_json::Error),
    #[error("serialized read model fragment is invalid")]
    InvalidFragment(#[from] SerializedReadModelFragmentError),
    #[error("serialized read model partition is invalid")]
    InvalidPartition(#[from] SerializedPartitionError),
    #[error("failed to deserialize a read model fragment")]
    DeserializeFragment(#[source] serde_json::Error),
    #[error("fragment type mismatch: expected {expected}, got {actual}")]
    FragmentMismatch { expected: String, actual: String },
}
