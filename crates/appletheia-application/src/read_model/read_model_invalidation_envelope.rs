use appletheia_domain::{EventId, EventOccurredAt};
use serde::{Deserialize, Serialize};

use crate::event::{EventEnvelope, EventSequence};
use crate::projection::{ProjectorName, ProjectorNameOwned};
use crate::request_context::{CausationId, CorrelationId};

use super::{ReadModelDependency, ReadModelInvalidationEnvelopeError, ReadModelInvalidationId};

/// Carries dependency keys invalidated by one committed projection update.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "UncheckedReadModelInvalidationEnvelope")]
pub struct ReadModelInvalidationEnvelope {
    pub invalidation_id: ReadModelInvalidationId,
    pub source_event_id: EventId,
    pub source_event_sequence: EventSequence,
    pub source_projector_name: ProjectorNameOwned,
    pub occurred_at: EventOccurredAt,
    pub correlation_id: CorrelationId,
    pub causation_id: CausationId,
    pub invalidated_dependencies: Vec<ReadModelDependency>,
}

#[derive(Deserialize)]
struct UncheckedReadModelInvalidationEnvelope {
    invalidation_id: ReadModelInvalidationId,
    source_event_id: EventId,
    source_event_sequence: EventSequence,
    source_projector_name: ProjectorNameOwned,
    occurred_at: EventOccurredAt,
    correlation_id: CorrelationId,
    causation_id: CausationId,
    invalidated_dependencies: Vec<ReadModelDependency>,
}

impl ReadModelInvalidationEnvelope {
    /// Creates an invalidation and removes duplicate dependency keys.
    pub fn try_new(
        event: &EventEnvelope,
        projector_name: ProjectorName,
        invalidated_dependencies: impl IntoIterator<Item = ReadModelDependency>,
    ) -> Result<Self, ReadModelInvalidationEnvelopeError> {
        let mut unique_dependencies = Vec::new();
        for dependency in invalidated_dependencies {
            if !unique_dependencies.contains(&dependency) {
                unique_dependencies.push(dependency);
            }
        }
        if unique_dependencies.is_empty() {
            return Err(ReadModelInvalidationEnvelopeError::EmptyDependencies);
        }

        Ok(Self {
            invalidation_id: ReadModelInvalidationId::new(),
            source_event_id: event.event_id,
            source_event_sequence: event.event_sequence,
            source_projector_name: ProjectorNameOwned::from(projector_name),
            occurred_at: event.occurred_at,
            correlation_id: event.correlation_id,
            causation_id: event.causation_id,
            invalidated_dependencies: unique_dependencies,
        })
    }
}

impl TryFrom<UncheckedReadModelInvalidationEnvelope> for ReadModelInvalidationEnvelope {
    type Error = ReadModelInvalidationEnvelopeError;

    fn try_from(value: UncheckedReadModelInvalidationEnvelope) -> Result<Self, Self::Error> {
        if value.invalidated_dependencies.is_empty() {
            return Err(ReadModelInvalidationEnvelopeError::EmptyDependencies);
        }
        Ok(Self {
            invalidation_id: value.invalidation_id,
            source_event_id: value.source_event_id,
            source_event_sequence: value.source_event_sequence,
            source_projector_name: value.source_projector_name,
            occurred_at: value.occurred_at,
            correlation_id: value.correlation_id,
            causation_id: value.causation_id,
            invalidated_dependencies: value.invalidated_dependencies,
        })
    }
}

#[cfg(test)]
mod tests {
    use appletheia_domain::AggregateVersion;
    use serde_json::json;
    use uuid::Uuid;

    use crate::event::{
        AggregateIdValue, AggregateTypeOwned, EventNameOwned, SerializedEventPayload,
    };
    use crate::read_model::SerializedPartition;
    use crate::request_context::{MessageId, Principal, RequestContext};

    use super::*;

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
    fn serializes_only_dependency_keys_and_deduplicates_them() {
        let dependency = ReadModelDependency::Partition(
            SerializedPartition::try_from(json!({ "fragment": "user", "key": 1 }))
                .expect("partition should be valid"),
        );
        let envelope = ReadModelInvalidationEnvelope::try_new(
            &event(),
            ProjectorName::new("test_projector"),
            [dependency.clone(), dependency],
        )
        .expect("invalidation should be valid");

        let value = serde_json::to_value(envelope).expect("invalidation should serialize");

        assert_eq!(
            value["invalidated_dependencies"].as_array().map(Vec::len),
            Some(1)
        );
        assert!(value.get("fragment").is_none());
        assert!(value.get("changes").is_none());
    }
}
