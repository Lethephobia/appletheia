use thiserror::Error;

use super::CloudEventPartitionKeyError;

#[derive(Debug, Error)]
pub enum CloudEventExtensionsError {
    #[error("partitionkey must be a nonempty String attribute")]
    InvalidPartitionKey,
    #[error(transparent)]
    CloudEventPartitionKey(#[from] CloudEventPartitionKeyError),
}
