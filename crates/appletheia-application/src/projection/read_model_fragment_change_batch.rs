use crate::event::EventEnvelope;
use crate::read_model::{
    ReadModelFragment, ReadModelFragmentChange, ReadModelFragmentChangeEnvelope,
    ReadModelFragmentChangeEnvelopeError, SerializedPartition, SerializedPartitionError,
};

use super::ProjectorName;

/// Collects ordered changes for one partition before durable delivery.
pub(crate) struct ReadModelFragmentChangeBatch<F>
where
    F: ReadModelFragment,
{
    partition: SerializedPartition,
    changes: Vec<ReadModelFragmentChange<F>>,
}

impl<F> ReadModelFragmentChangeBatch<F>
where
    F: ReadModelFragment,
{
    /// Starts one partition-scoped batch with its first change.
    pub(crate) fn try_new(
        change: ReadModelFragmentChange<F>,
    ) -> Result<Self, SerializedPartitionError> {
        let partition = change.partition().try_into_serialized::<F>()?;
        Ok(Self {
            partition,
            changes: vec![change],
        })
    }

    /// Reports whether this batch owns one serialized partition.
    pub(crate) fn has_partition(&self, partition: &SerializedPartition) -> bool {
        &self.partition == partition
    }

    /// Appends a later change for this same partition.
    pub(crate) fn push(&mut self, change: ReadModelFragmentChange<F>) {
        self.changes.push(change);
    }

    /// Finalizes this partition's ordered changes for durable delivery.
    pub(crate) fn try_into_envelope(
        self,
        event: &EventEnvelope,
        projector_name: ProjectorName,
    ) -> Result<ReadModelFragmentChangeEnvelope, ReadModelFragmentChangeEnvelopeError> {
        ReadModelFragmentChangeEnvelope::from_changes(self.changes, event, projector_name)
    }
}
