use serde::{Deserialize, Serialize};

use super::{ReadModelDependencyTopic, SerializedPartition};

/// Identifies one exact or prospective input that can invalidate a query.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ReadModelDependency {
    /// Watches one physical Fragment partition.
    Partition(SerializedPartition),
    /// Watches a prospective set of Fragment partitions.
    Topic(ReadModelDependencyTopic),
}

impl From<SerializedPartition> for ReadModelDependency {
    fn from(partition: SerializedPartition) -> Self {
        Self::Partition(partition)
    }
}

impl From<ReadModelDependencyTopic> for ReadModelDependency {
    fn from(topic: ReadModelDependencyTopic) -> Self {
        Self::Topic(topic)
    }
}
