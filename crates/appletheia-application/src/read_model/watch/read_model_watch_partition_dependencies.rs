use serde::{Deserialize, Serialize};

use crate::read_model::SerializedPartition;

/// Replaces the referenced fragments required by one watched fragment.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReadModelWatchPartitionDependencies {
    pub partition: SerializedPartition,
    pub referenced_partitions: Vec<SerializedPartition>,
}
