use crate::event::EventEnvelope;
use crate::read_model::{
    ReadModelFragment, ReadModelFragmentChange, ReadModelFragmentChangeEnvelope,
    ReadModelFragmentChangeEnvelopeError, SerializedPartitionError,
};

use super::ProjectorName;
use super::read_model_fragment_change_batch::ReadModelFragmentChangeBatch;

/// Collects ordered partition-scoped fragment-change batches from one projector run.
pub(crate) struct ReadModelFragmentChangeBatches<F>
where
    F: ReadModelFragment,
{
    batches: Vec<ReadModelFragmentChangeBatch<F>>,
}

impl<F> ReadModelFragmentChangeBatches<F>
where
    F: ReadModelFragment,
{
    /// Groups ordered projector changes by their physical partition.
    pub(crate) fn from_changes(
        changes: Vec<ReadModelFragmentChange<F>>,
    ) -> Result<Self, SerializedPartitionError> {
        let mut batches: Vec<ReadModelFragmentChangeBatch<F>> = Vec::new();
        for change in changes {
            let partition = change.partition().try_into_serialized::<F>()?;
            if let Some(batch) = batches
                .iter_mut()
                .find(|batch| batch.has_partition(&partition))
            {
                batch.push(change);
            } else {
                batches.push(ReadModelFragmentChangeBatch::try_new(change)?);
            }
        }

        Ok(Self { batches })
    }

    /// Finalizes each partition-scoped batch for durable delivery.
    pub(crate) fn try_into_envelopes(
        self,
        event: &EventEnvelope,
        projector_name: ProjectorName,
    ) -> Result<Vec<ReadModelFragmentChangeEnvelope>, ReadModelFragmentChangeEnvelopeError> {
        self.batches
            .into_iter()
            .map(|batch| batch.try_into_envelope(event, projector_name))
            .collect()
    }
}
