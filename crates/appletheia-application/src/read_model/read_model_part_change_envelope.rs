use appletheia_domain::{EventId, EventOccurredAt};
use serde::{Deserialize, Serialize};

use crate::event::{AggregateIdValue, AggregateTypeOwned, EventSequence};
use crate::projection::ProjectorNameOwned;
use crate::request_context::{CausationId, CorrelationId};

use super::{
    ReadModelFragmentChangeEnvelope, ReadModelFragmentChangeId, ReadModelNameOwned,
    ReadModelPartChange, ReadModelPartChangeEnvelopeError, SerializedPartition,
};

/// Carries client-facing part replacements derived for one read model and source partition.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReadModelPartChangeEnvelope {
    pub read_model_name: ReadModelNameOwned,
    pub source_change_id: ReadModelFragmentChangeId,
    pub source_partition: SerializedPartition,
    pub source_projector_name: ProjectorNameOwned,
    pub source_event_sequence: EventSequence,
    pub source_event_id: EventId,
    pub source_aggregate_type: AggregateTypeOwned,
    pub source_aggregate_id: AggregateIdValue,
    pub occurred_at: EventOccurredAt,
    pub correlation_id: CorrelationId,
    pub causation_id: CausationId,
    pub part_changes: Vec<ReadModelPartChange>,
}

impl ReadModelPartChangeEnvelope {
    /// Attaches one read model identity to mapped part changes.
    pub fn try_from_fragment_envelope(
        source: &ReadModelFragmentChangeEnvelope,
        read_model_name: ReadModelNameOwned,
        part_changes: Vec<ReadModelPartChange>,
    ) -> Result<Self, ReadModelPartChangeEnvelopeError> {
        if part_changes.is_empty() {
            return Err(ReadModelPartChangeEnvelopeError::EmptyChanges);
        }

        Ok(Self {
            read_model_name,
            source_change_id: source.change_id,
            source_partition: source.partition.clone(),
            source_projector_name: source.source_projector_name.clone(),
            source_event_sequence: source.source_event_sequence,
            source_event_id: source.source_event_id,
            source_aggregate_type: source.source_aggregate_type.clone(),
            source_aggregate_id: source.source_aggregate_id,
            occurred_at: source.occurred_at,
            correlation_id: source.correlation_id,
            causation_id: source.causation_id,
            part_changes,
        })
    }
}
