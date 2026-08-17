use crate::read_model::{ReadModelPartChangeEnvelope, SerializedPartition};

use super::ReadModelWatchPartitionDependencies;

/// Contains the filtered effects of one change envelope on one subscription.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelWatchRoute {
    pub change: Option<ReadModelPartChangeEnvelope>,
    pub list_invalidated: bool,
    /// Partitions that must be installed before the routed response is emitted.
    pub partitions_to_add: Vec<SerializedPartition>,
    /// Direct source partitions that must be removed before the response is emitted.
    pub partitions_to_remove: Vec<SerializedPartition>,
    /// Complete dependency replacements for fragments visible in this delivery.
    pub dependency_replacements: Vec<ReadModelWatchPartitionDependencies>,
}
