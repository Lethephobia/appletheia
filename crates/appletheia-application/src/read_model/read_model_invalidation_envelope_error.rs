use thiserror::Error;

use crate::messaging::ReadModelInvalidationCloudEventCodecError;

use super::SerializedPartitionError;

/// Reports an invalid durable read-model invalidation envelope.
#[derive(Debug, Error)]
pub enum ReadModelInvalidationEnvelopeError {
    #[error(transparent)]
    ReadModelInvalidationCloudEventCodec(#[from] ReadModelInvalidationCloudEventCodecError),

    #[error("a read-model invalidation must contain at least one partition")]
    EmptyPartitions,

    #[error(transparent)]
    SerializedPartition(#[from] SerializedPartitionError),
}
