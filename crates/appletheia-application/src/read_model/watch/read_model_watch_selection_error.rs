use thiserror::Error;

use crate::read_model::SerializedPartitionError;

/// Reports a failure to serialize a read model's watch selection.
#[derive(Debug, Error)]
pub enum ReadModelWatchSelectionError {
    #[error(transparent)]
    InvalidPartition(#[from] SerializedPartitionError),
}
