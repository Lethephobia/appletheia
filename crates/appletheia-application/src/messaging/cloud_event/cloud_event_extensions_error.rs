use thiserror::Error;

use super::PartitionKeyError;

#[derive(Debug, Error)]
pub enum CloudEventExtensionsError {
    #[error("partitionkey must be a nonempty String attribute")]
    InvalidPartitionKey,
    #[error(transparent)]
    PartitionKey(#[from] PartitionKeyError),
}
