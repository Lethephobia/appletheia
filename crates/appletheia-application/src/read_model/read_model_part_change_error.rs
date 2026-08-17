use thiserror::Error;

use super::{
    ReadModelFragmentChangeError, ReadModelPartPathError, SerializedPartitionError,
    SerializedReadModelPartError,
};

/// Reports a failure while building or reading a read model part change.
#[derive(Debug, Error)]
pub enum ReadModelPartChangeError {
    #[error("physical fragment change is invalid")]
    Fragment(#[from] ReadModelFragmentChangeError),
    #[error("failed to serialize a read model part")]
    SerializePart(#[source] serde_json::Error),
    #[error("serialized read model part is invalid")]
    InvalidPart(#[from] SerializedReadModelPartError),
    #[error("serialized read model partition is invalid")]
    InvalidPartition(#[from] SerializedPartitionError),
    #[error("read model part replacement path is invalid")]
    InvalidPath(#[from] ReadModelPartPathError),
    #[error("failed to deserialize a read model part")]
    DeserializePart(#[source] serde_json::Error),
}
