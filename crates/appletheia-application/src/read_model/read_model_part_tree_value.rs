use super::{ReadModelPart, ReadModelPartTree, SerializedPartition, SerializedPartitionError};

/// Holds one materialized part's source partition and child part trees.
pub(super) struct ReadModelPartTreeValue {
    pub(super) partition: Result<SerializedPartition, SerializedPartitionError>,
    pub(super) children: Vec<ReadModelPartTree>,
}

impl ReadModelPartTreeValue {
    pub(super) fn new<P>(part: &P) -> Self
    where
        P: ReadModelPart,
    {
        Self {
            partition: part.partition().try_into_serialized::<P::SourceFragment>(),
            children: P::parts(Some(part)),
        }
    }
}
