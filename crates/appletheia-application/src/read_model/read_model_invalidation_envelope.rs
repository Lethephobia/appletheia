use crate::messaging::{CloudEvent, CloudEventSource, CloudEventTime, CloudEventTypePrefix};
use crate::messaging::{
    CloudEventAttributeValue, CloudEventData, CloudEventDataContentType, CloudEventPartitionKey,
    CloudEventType,
};
use crate::request_context::MessageId;
use appletheia_domain::{EventId, EventOccurredAt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::event::{EventEnvelope, EventSequence};
use crate::messaging::PublishableMessage;
use crate::projection::{ProjectorName, ProjectorNameOwned};
use crate::request_context::{CausationId, CorrelationId};

use super::{
    ReadModelFragment, ReadModelInvalidatedPartitions, ReadModelInvalidationEnvelopeError,
    ReadModelInvalidationId, SerializedPartition,
};

/// Carries partition keys invalidated by one committed projection update.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReadModelInvalidationEnvelope {
    pub invalidation_id: ReadModelInvalidationId,
    pub source_event_id: EventId,
    pub source_event_sequence: EventSequence,
    pub source_projector_name: ProjectorNameOwned,
    pub source_event_occurred_at: EventOccurredAt,
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
            invalidation_id: ReadModelInvalidationId::new(),
            source_event_id: event.event_id,
            source_event_sequence: event.event_sequence,
            source_projector_name: ProjectorNameOwned::from(projector_name),
            source_event_occurred_at: event.occurred_at,
            correlation_id: event.correlation_id,
            causation_id: CausationId::from(event.event_id),
            invalidated_partitions: serialized_partitions,
        })
    }
}

impl PublishableMessage for ReadModelInvalidationEnvelope {
    type Error = ReadModelInvalidationEnvelopeError;

    fn try_to_cloud_event(
        &self,
        source: &CloudEventSource,
        type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CloudEvent, Self::Error> {
        if self.source_event_id.value() != self.causation_id.value() {
            return Err(ReadModelInvalidationEnvelopeError::InvalidMetadata(
                "source_event_id/causationid",
            ));
        }
        if self.invalidated_partitions.is_empty() {
            return Err(ReadModelInvalidationEnvelopeError::EmptyPartitions);
        }
        let mut event = CloudEvent::new(
            self.invalidation_id.to_string().parse()?,
            source.clone(),
            CloudEventType::with_prefix(type_prefix, "read_model.invalidated")?,
        )
        .with_partition_key(CloudEventPartitionKey::new(
            self.source_projector_name.to_string(),
        )?);
        event.replace_data(
            Some(CloudEventData::Json(
                serde_json::json!({"invalidated_partitions": self.invalidated_partitions}),
            )),
            Some(CloudEventDataContentType::json()),
        )?;
        for (name, value) in [
            ("correlationid", self.correlation_id.to_string()),
            ("causationid", self.causation_id.to_string()),
            ("sourceeventid", self.source_event_id.to_string()),
            (
                "sourceeventsequence",
                self.source_event_sequence.to_string(),
            ),
            (
                "sourceprojectorname",
                self.source_projector_name.to_string(),
            ),
        ] {
            event.insert_extension(
                name.parse()?,
                CloudEventAttributeValue::String(value.parse()?),
            )?;
        }
        event.insert_extension(
            "sourceeventoccurredat".parse()?,
            CloudEventAttributeValue::Timestamp(CloudEventTime::new(
                self.source_event_occurred_at.into(),
            )?),
        )?;
        Ok(event)
    }

    fn try_from_cloud_event(
        event: &CloudEvent,
        type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<Self, Self::Error> {
        if !event
            .data_content_type()
            .is_some_and(CloudEventDataContentType::is_json)
        {
            return Err(ReadModelInvalidationEnvelopeError::InvalidMetadata(
                "datacontenttype",
            ));
        }
        let data = match event.data() {
            Some(CloudEventData::Json(value)) => value.clone(),
            Some(CloudEventData::Binary(bytes)) => serde_json::from_slice(bytes)?,
            Some(CloudEventData::Text(text)) => serde_json::from_str(text)?,
            None => return Err(ReadModelInvalidationEnvelopeError::InvalidMetadata("data")),
        };
        let name = event.event_type().without_prefix(type_prefix)?;
        if name != "read_model.invalidated" {
            return Err(ReadModelInvalidationEnvelopeError::InvalidMetadata("type"));
        }
        let envelope = Self {
            invalidation_id: ReadModelInvalidationId::try_from(
                event.id().as_str().parse::<Uuid>()?,
            )?,
            source_event_id: EventId::try_from(
                event
                    .extensions()
                    .get(&"sourceeventid".parse()?)
                    .ok_or(ReadModelInvalidationEnvelopeError::InvalidMetadata(
                        "sourceeventid",
                    ))?
                    .to_string()
                    .parse::<Uuid>()?,
            )?,
            source_event_sequence: EventSequence::try_from(
                event
                    .extensions()
                    .get(&"sourceeventsequence".parse()?)
                    .ok_or(ReadModelInvalidationEnvelopeError::InvalidMetadata(
                        "sourceeventsequence",
                    ))?
                    .to_string()
                    .parse::<i64>()?,
            )?,
            source_projector_name: ProjectorNameOwned::new(
                event
                    .extensions()
                    .get(&"sourceprojectorname".parse()?)
                    .ok_or(ReadModelInvalidationEnvelopeError::InvalidMetadata(
                        "sourceprojectorname",
                    ))?
                    .to_string(),
            )?,
            source_event_occurred_at: event
                .extensions()
                .get(&"sourceeventoccurredat".parse()?)
                .ok_or(ReadModelInvalidationEnvelopeError::InvalidMetadata(
                    "sourceeventoccurredat",
                ))?
                .to_string()
                .parse::<CloudEventTime>()?
                .value()
                .into(),
            invalidated_partitions: serde_json::from_value(
                data.get("invalidated_partitions").cloned().ok_or(
                    ReadModelInvalidationEnvelopeError::InvalidMetadata("invalidated_partitions"),
                )?,
            )?,
            correlation_id: CorrelationId::from(
                event
                    .extensions()
                    .get(&"correlationid".parse()?)
                    .ok_or(ReadModelInvalidationEnvelopeError::InvalidMetadata(
                        "correlationid",
                    ))?
                    .to_string()
                    .parse::<Uuid>()?,
            ),
            causation_id: CausationId::from(MessageId::from(
                event
                    .extensions()
                    .get(&"causationid".parse()?)
                    .ok_or(ReadModelInvalidationEnvelopeError::InvalidMetadata(
                        "causationid",
                    ))?
                    .to_string()
                    .parse::<Uuid>()?,
            )),
        };
        if envelope.source_event_id.value() != envelope.causation_id.value() {
            return Err(ReadModelInvalidationEnvelopeError::InvalidMetadata(
                "source_event_id/causationid",
            ));
        }
        if envelope.invalidated_partitions.is_empty() {
            return Err(ReadModelInvalidationEnvelopeError::EmptyPartitions);
        }
        if event
            .partition_key()
            .as_ref()
            .map(CloudEventPartitionKey::as_str)
            != Some((envelope.source_projector_name.to_string()).as_str())
        {
            return Err(ReadModelInvalidationEnvelopeError::InvalidMetadata(
                "partitionkey",
            ));
        }
        Ok(envelope)
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
        assert!(value.get("invalidation_id").is_some());

        let restored: ReadModelInvalidationEnvelope =
            serde_json::from_value(value.clone()).expect("nonempty partitions should deserialize");
        assert_eq!(restored, envelope);
    }

    #[test]
    fn cloud_event_keeps_notification_identity_distinct_from_source_event() {
        let source_event = event();
        let mut partitions = ReadModelInvalidatedPartitions::new();
        partitions.insert(1);
        let envelope = ReadModelInvalidationEnvelope::try_new::<TestFragment>(
            &source_event,
            ProjectorName::new("test_projector"),
            partitions,
        )
        .unwrap();
        assert_ne!(
            envelope.invalidation_id.value(),
            source_event.event_id.value()
        );
        assert_eq!(envelope.causation_id.value(), source_event.event_id.value());
        assert_eq!(envelope.source_event_occurred_at, source_event.occurred_at);
        let source = "urn:banking:invalidations".parse().unwrap();
        let mut cloud_event = envelope.try_to_cloud_event(&source, None).unwrap();
        assert_eq!(
            cloud_event.id().as_str(),
            envelope.invalidation_id.to_string()
        );
        assert!(cloud_event.time().is_none());
        assert!(cloud_event.subject().is_none());
        assert_eq!(
            ReadModelInvalidationEnvelope::try_from_cloud_event(&cloud_event, None).unwrap(),
            envelope
        );
        assert_eq!(
            envelope.try_to_cloud_event(&source, None).unwrap(),
            cloud_event
        );
        cloud_event
            .insert_extension(
                "causationid".parse().unwrap(),
                CloudEventAttributeValue::String((MessageId::new().to_string()).parse().unwrap()),
            )
            .unwrap();
        assert!(ReadModelInvalidationEnvelope::try_from_cloud_event(&cloud_event, None).is_err());
    }
}
