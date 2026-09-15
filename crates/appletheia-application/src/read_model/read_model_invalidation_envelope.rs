use appletheia_domain::{EventId, EventOccurredAt};
use serde::{Deserialize, Serialize};

use crate::event::{EventEnvelope, EventSequence};
use crate::messaging::{OrderingKey, PublishableMessage};
use crate::projection::{ProjectorName, ProjectorNameOwned};
use crate::request_context::{CausationId, CorrelationId};

use super::{
    ReadModelFragment, ReadModelInvalidatedPartitions, ReadModelInvalidationEnvelopeError,
    SerializedPartition,
};

/// Carries partition keys invalidated by one committed projection update.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReadModelInvalidationEnvelope {
    pub source_event_id: EventId,
    pub source_event_sequence: EventSequence,
    pub source_projector_name: ProjectorNameOwned,
    pub occurred_at: EventOccurredAt,
    pub correlation_id: CorrelationId,
    pub causation_id: CausationId,
    pub invalidated_partitions: Vec<SerializedPartition>,
}

impl ReadModelInvalidationEnvelope {
    /// Serializes invalidated partitions and attaches the source event metadata.
    pub fn try_new<F>(
        event: &EventEnvelope,
        projector_name: ProjectorName,
        invalidated_partitions: ReadModelInvalidatedPartitions<F::Key>,
    ) -> Result<Self, ReadModelInvalidationEnvelopeError>
    where
        F: ReadModelFragment,
    {
        if invalidated_partitions.is_empty() {
            return Err(ReadModelInvalidationEnvelopeError::EmptyPartitions);
        }
        let serialized_partitions = invalidated_partitions
            .into_iter()
            .map(|partition| partition.try_into_serialized::<F>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            source_event_id: event.event_id,
            source_event_sequence: event.event_sequence,
            source_projector_name: ProjectorNameOwned::from(projector_name),
            occurred_at: event.occurred_at,
            correlation_id: event.correlation_id,
            causation_id: event.causation_id,
            invalidated_partitions: serialized_partitions,
        })
    }
}

impl PublishableMessage for ReadModelInvalidationEnvelope {
    fn ordering_key(&self) -> OrderingKey {
        OrderingKey::from(&self.source_projector_name)
    }
}

#[cfg(test)]
mod tests {
    use appletheia_domain::AggregateVersion;
    use serde_json::json;
    use uuid::Uuid;

    use crate::aggregate::{AggregateIdValue, AggregateTypeOwned};
    use crate::event::{EventNameOwned, SerializedEventPayload};
    use crate::read_model::{
        ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
    };
    use crate::request_context::{MessageId, Principal, RequestContext};

    use super::*;

    struct TestFragment;

    impl ReadModelObservationSource for TestFragment {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModelFragment for TestFragment {
        const NAME: ReadModelFragmentName = ReadModelFragmentName::new("test");
        type Key = u64;

        fn key(&self) -> Self::Key {
            1
        }
    }

    fn event() -> EventEnvelope {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let message_id = MessageId::new();
        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::try_from("test")
                .expect("aggregate type should be valid"),
            aggregate_id: AggregateIdValue::from(Uuid::now_v7()),
            aggregate_version: AggregateVersion::try_from(1)
                .expect("aggregate version should be valid"),
            event_name: EventNameOwned::try_from("changed").expect("event name should be valid"),
            payload: SerializedEventPayload::try_from(json!({})).expect("payload should be valid"),
            occurred_at: EventOccurredAt::now(),
            correlation_id,
            causation_id: CausationId::from(message_id),
            context: RequestContext::new(correlation_id, message_id, Principal::System)
                .expect("request context should be valid"),
        }
    }

    #[test]
    fn rejects_empty_partitions_at_the_delivery_boundary() {
        let result = ReadModelInvalidationEnvelope::try_new::<TestFragment>(
            &event(),
            ProjectorName::new("test_projector"),
            ReadModelInvalidatedPartitions::new(),
        );
        assert!(matches!(
            result,
            Err(ReadModelInvalidationEnvelopeError::EmptyPartitions)
        ));
    }

    #[test]
    fn serializes_typed_partitions_with_fragment_identity() {
        let mut partitions = ReadModelInvalidatedPartitions::new();
        partitions.insert(1);
        let envelope = ReadModelInvalidationEnvelope::try_new::<TestFragment>(
            &event(),
            ProjectorName::new("test_projector"),
            partitions,
        )
        .expect("invalidation should be valid");

        let value = serde_json::to_value(&envelope).expect("invalidation should serialize");

        assert_eq!(
            value["invalidated_partitions"].as_array().map(Vec::len),
            Some(1)
        );
        assert_eq!(
            value["invalidated_partitions"][0],
            json!({"fragment_name": "test", "key": 1})
        );
        assert!(value.get("fragment").is_none());
        assert!(value.get("changes").is_none());
        assert!(value.get("invalidation_id").is_none());

        let restored: ReadModelInvalidationEnvelope =
            serde_json::from_value(value.clone()).expect("nonempty partitions should deserialize");
        assert_eq!(restored, envelope);
    }
}
