use appletheia_domain::{EventId, EventOccurredAt};
use serde::{Deserialize, Serialize};

use crate::event::{AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventSequence};
use crate::projection::{ProjectorName, ProjectorNameOwned};
use crate::request_context::{CausationId, CorrelationId};

use super::{
    ReadModelFragment, ReadModelFragmentChange, ReadModelFragmentChangeEnvelopeError,
    ReadModelFragmentChangeError, ReadModelFragmentChangeId, SerializedPartition,
    SerializedReadModelFragmentChange,
};

/// Carries ordered source-fragment changes for one partition through durable delivery.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "UncheckedReadModelFragmentChangeEnvelope")]
pub struct ReadModelFragmentChangeEnvelope {
    pub change_id: ReadModelFragmentChangeId,
    pub partition: SerializedPartition,
    pub source_projector_name: ProjectorNameOwned,
    pub source_event_sequence: EventSequence,
    pub source_event_id: EventId,
    pub source_aggregate_type: AggregateTypeOwned,
    pub source_aggregate_id: AggregateIdValue,
    pub occurred_at: EventOccurredAt,
    pub correlation_id: CorrelationId,
    pub causation_id: CausationId,
    pub changes: Vec<SerializedReadModelFragmentChange>,
}

#[derive(Deserialize)]
struct UncheckedReadModelFragmentChangeEnvelope {
    change_id: ReadModelFragmentChangeId,
    partition: SerializedPartition,
    source_projector_name: ProjectorNameOwned,
    source_event_sequence: EventSequence,
    source_event_id: EventId,
    source_aggregate_type: AggregateTypeOwned,
    source_aggregate_id: AggregateIdValue,
    occurred_at: EventOccurredAt,
    correlation_id: CorrelationId,
    causation_id: CausationId,
    changes: Vec<SerializedReadModelFragmentChange>,
}

impl ReadModelFragmentChangeEnvelope {
    /// Finalizes ordered typed projector changes for one partition.
    pub fn from_changes<F>(
        changes: Vec<ReadModelFragmentChange<F>>,
        event: &EventEnvelope,
        projector_name: ProjectorName,
    ) -> Result<Self, ReadModelFragmentChangeEnvelopeError>
    where
        F: ReadModelFragment,
    {
        let Some(first_change) = changes.first() else {
            return Err(ReadModelFragmentChangeEnvelopeError::EmptyChanges);
        };
        let partition = first_change
            .partition()
            .try_into_serialized::<F>()
            .map_err(ReadModelFragmentChangeError::from)?;
        let serialized_changes = changes
            .into_iter()
            .map(|change| {
                let change_partition = change
                    .partition()
                    .try_into_serialized::<F>()
                    .map_err(ReadModelFragmentChangeError::from)?;
                if change_partition != partition {
                    return Err(ReadModelFragmentChangeEnvelopeError::PartitionMismatch);
                }
                change.try_into_serialized().map_err(Into::into)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            change_id: ReadModelFragmentChangeId::new(),
            partition,
            source_projector_name: ProjectorNameOwned::from(projector_name),
            source_event_sequence: event.event_sequence,
            source_event_id: event.event_id,
            source_aggregate_type: event.aggregate_type.clone(),
            source_aggregate_id: event.aggregate_id,
            occurred_at: event.occurred_at,
            correlation_id: event.correlation_id,
            causation_id: event.causation_id,
            changes: serialized_changes,
        })
    }
}

impl TryFrom<UncheckedReadModelFragmentChangeEnvelope> for ReadModelFragmentChangeEnvelope {
    type Error = ReadModelFragmentChangeEnvelopeError;

    fn try_from(value: UncheckedReadModelFragmentChangeEnvelope) -> Result<Self, Self::Error> {
        if value.changes.is_empty() {
            return Err(ReadModelFragmentChangeEnvelopeError::EmptyChanges);
        }
        if value
            .changes
            .iter()
            .any(|change| change.partition() != &value.partition)
        {
            return Err(ReadModelFragmentChangeEnvelopeError::PartitionMismatch);
        }

        Ok(Self {
            change_id: value.change_id,
            partition: value.partition,
            source_projector_name: value.source_projector_name,
            source_event_sequence: value.source_event_sequence,
            source_event_id: value.source_event_id,
            source_aggregate_type: value.source_aggregate_type,
            source_aggregate_id: value.source_aggregate_id,
            occurred_at: value.occurred_at,
            correlation_id: value.correlation_id,
            causation_id: value.causation_id,
            changes: value.changes,
        })
    }
}

#[cfg(test)]
mod tests {
    use appletheia_domain::{AggregateVersion, EventOccurredAt};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use super::*;
    use crate::event::{EventEnvelope, EventNameOwned, SerializedEventPayload};
    use crate::projection::ProjectorName;
    use crate::read_model::{
        ReadModelFragmentChange, ReadModelFragmentName, ReadModelObservation,
        ReadModelObservationSource,
    };
    use crate::request_context::{MessageId, Principal, RequestContext};

    #[derive(Clone, Deserialize, Serialize)]
    struct TestFragment {
        id: Uuid,
    }

    impl ReadModelObservationSource for TestFragment {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModelFragment for TestFragment {
        const NAME: ReadModelFragmentName = ReadModelFragmentName::new("test_fragment");
        type Key = Uuid;

        fn key(&self) -> Self::Key {
            self.id
        }
    }

    fn event() -> EventEnvelope {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let message_id = MessageId::new();
        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::try_from("test_aggregate")
                .expect("aggregate type should be valid"),
            aggregate_id: AggregateIdValue::from(Uuid::now_v7()),
            aggregate_version: AggregateVersion::try_from(1)
                .expect("aggregate version should be valid"),
            event_name: EventNameOwned::try_from("changed").expect("event name should be valid"),
            payload: SerializedEventPayload::try_from(serde_json::json!({}))
                .expect("payload should be valid"),
            occurred_at: EventOccurredAt::now(),
            correlation_id,
            causation_id: CausationId::from(message_id),
            context: RequestContext::new(correlation_id, message_id, Principal::System)
                .expect("request context should be valid"),
        }
    }

    #[test]
    fn finalizes_typed_changes_as_type_erased_envelopes() {
        let fragment = TestFragment { id: Uuid::now_v7() };
        let change = ReadModelFragmentChange::Changed(fragment);
        let envelope = ReadModelFragmentChangeEnvelope::from_changes(
            vec![change],
            &event(),
            ProjectorName::new("test_projector"),
        )
        .expect("fragment change should finalize");

        assert_eq!(
            envelope.changes[0].fragment_name().value(),
            TestFragment::NAME.value()
        );
        assert_eq!(envelope.changes[0].partition(), &envelope.partition);
    }

    #[test]
    fn preserves_ordered_changes_for_one_partition() {
        let id = Uuid::now_v7();
        let first_change = ReadModelFragmentChange::Changed(TestFragment { id });
        let second_change = ReadModelFragmentChange::Removed(id);

        let envelope = ReadModelFragmentChangeEnvelope::from_changes(
            vec![first_change, second_change],
            &event(),
            ProjectorName::new("test_projector"),
        )
        .expect("fragment changes should finalize");

        assert_eq!(envelope.changes.len(), 2);
        assert!(matches!(
            &envelope.changes[0],
            SerializedReadModelFragmentChange::Changed { .. }
        ));
        assert!(matches!(
            &envelope.changes[1],
            SerializedReadModelFragmentChange::Removed { .. }
        ));
    }

    #[test]
    fn rejects_a_type_erased_change_with_a_different_partition() {
        let serialized_change =
            ReadModelFragmentChange::try_from_fragment(&TestFragment { id: Uuid::now_v7() })
                .expect("test fragment should serialize")
                .try_into_serialized()
                .expect("test fragment should serialize");
        let partition = serialized_change.partition().clone();
        let different_partition = SerializedPartition::try_from(serde_json::json!({
            "key": Uuid::now_v7(),
        }))
        .expect("partition should serialize");
        let fragment_change = match serialized_change {
            SerializedReadModelFragmentChange::Changed {
                fragment_name,
                fragment,
                ..
            } => SerializedReadModelFragmentChange::Changed {
                fragment_name,
                partition: different_partition,
                fragment,
            },
            SerializedReadModelFragmentChange::Removed { .. } => {
                unreachable!("a fragment replacement should be changed")
            }
        };
        let event = event();
        let unchecked = UncheckedReadModelFragmentChangeEnvelope {
            change_id: ReadModelFragmentChangeId::new(),
            partition,
            source_projector_name: ProjectorNameOwned::from(ProjectorName::new("test_projector")),
            source_event_sequence: event.event_sequence,
            source_event_id: event.event_id,
            source_aggregate_type: event.aggregate_type,
            source_aggregate_id: event.aggregate_id,
            occurred_at: event.occurred_at,
            correlation_id: event.correlation_id,
            causation_id: event.causation_id,
            changes: vec![fragment_change],
        };

        assert!(matches!(
            ReadModelFragmentChangeEnvelope::try_from(unchecked),
            Err(ReadModelFragmentChangeEnvelopeError::PartitionMismatch)
        ));
    }
}
