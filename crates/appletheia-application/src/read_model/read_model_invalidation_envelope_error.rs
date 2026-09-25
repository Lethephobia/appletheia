use thiserror::Error;

use super::SerializedPartitionError;

/// Reports an invalid durable read-model invalidation envelope.
#[derive(Debug, Error)]
pub enum ReadModelInvalidationEnvelopeError {
    #[error("a read-model invalidation must contain at least one partition")]
    EmptyPartitions,

    #[error(transparent)]
    SerializedPartition(#[from] SerializedPartitionError),
}
