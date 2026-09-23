use appletheia_domain::{AggregateVersion, EventId};
use uuid::Uuid;

use crate::aggregate::{AggregateIdValue, AggregateTypeOwned};
use crate::event::{EventEnvelope, EventNameOwned, EventSequence, SerializedEventPayload};
use crate::messaging::{
    CloudEvent, CloudEventAttributeValue, CloudEventData, CloudEventDataContentType, CloudEventId,
    CloudEventPartitionKey, CloudEventSource, CloudEventTime, CloudEventType, CloudEventTypePrefix,
};
use crate::request_context::{CausationId, CorrelationId, MessageId};

use super::EventCloudEventCodecError;

pub struct EventCloudEventCodec;

impl EventCloudEventCodec {
    pub fn encode(
        envelope: &EventEnvelope,
        source: &CloudEventSource,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CloudEvent, EventCloudEventCodecError> {
        let mut event = CloudEvent::new(
            Self::encode_id(envelope.event_id)?,
            source.clone(),
            Self::encode_type(
                cloud_event_type_prefix,
                &envelope.aggregate_type,
                &envelope.event_name,
            )?,
        )
        .with_partition_key(CloudEventPartitionKey::new(format!(
            "{}:{}",
            envelope.aggregate_type, envelope.aggregate_id
        ))?)
        .try_with_data(
            Some(CloudEventData::Json(envelope.payload.value().clone())),
            Some(CloudEventDataContentType::json()),
        )?
        .with_subject(format!("{}/{}", envelope.aggregate_type, envelope.aggregate_id).parse()?)
        .with_time(CloudEventTime::new(envelope.occurred_at.into())?);
        for (name, value) in [
            ("correlationid", envelope.correlation_id.to_string()),
            ("causationid", envelope.causation_id.to_string()),
            ("aggregateversion", envelope.aggregate_version.to_string()),
            ("eventsequence", envelope.event_sequence.to_string()),
            ("context", serde_json::to_string(&envelope.context)?),
        ] {
            event.insert_extension(
                name.parse()?,
                CloudEventAttributeValue::String(value.parse()?),
            )?;
        }
        Ok(event)
    }

    pub fn decode(
        event: &CloudEvent,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<EventEnvelope, EventCloudEventCodecError> {
        if !event
            .data_content_type()
            .is_some_and(CloudEventDataContentType::is_json)
        {
            return Err(EventCloudEventCodecError::InvalidMetadata(
                "datacontenttype",
            ));
        }
        let data = match event.data() {
            Some(CloudEventData::Json(value)) => value.clone(),
            Some(CloudEventData::Binary(bytes)) => serde_json::from_slice(bytes)?,
            Some(CloudEventData::Text(text)) => serde_json::from_str(text)?,
            None => return Err(EventCloudEventCodecError::InvalidMetadata("data")),
        };
        let (aggregate_type, event_name) =
            Self::decode_type(event.event_type(), cloud_event_type_prefix)?;
        let subject = event
            .subject()
            .ok_or(EventCloudEventCodecError::InvalidMetadata("subject"))?
            .as_str();
        let (subject_aggregate_type, aggregate_id) = subject
            .rsplit_once('/')
            .ok_or(EventCloudEventCodecError::InvalidMetadata("subject"))?;
        if aggregate_type.value() != subject_aggregate_type {
            return Err(EventCloudEventCodecError::InvalidMetadata("type/subject"));
        }
        let envelope = EventEnvelope {
            event_id: Self::decode_id(event.id())?,
            event_sequence: EventSequence::try_from(
                event
                    .extensions()
                    .get(&"eventsequence".parse()?)
                    .ok_or(EventCloudEventCodecError::InvalidMetadata("eventsequence"))?
                    .to_string()
                    .parse::<i64>()?,
            )?,
            aggregate_type,
            aggregate_id: AggregateIdValue::from(aggregate_id.parse::<Uuid>()?),
            aggregate_version: AggregateVersion::try_from(
                event
                    .extensions()
                    .get(&"aggregateversion".parse()?)
                    .ok_or(EventCloudEventCodecError::InvalidMetadata(
                        "aggregateversion",
                    ))?
                    .to_string()
                    .parse::<i64>()?,
            )?,
            event_name,
            payload: SerializedEventPayload::try_from(data)?,
            occurred_at: event
                .time()
                .ok_or(EventCloudEventCodecError::InvalidMetadata("time"))?
                .value()
                .into(),
            context: serde_json::from_str(
                &event
                    .extensions()
                    .get(&"context".parse()?)
                    .ok_or(EventCloudEventCodecError::InvalidMetadata("context"))?
                    .to_string(),
            )?,
            correlation_id: CorrelationId::from(
                event
                    .extensions()
                    .get(&"correlationid".parse()?)
                    .ok_or(EventCloudEventCodecError::InvalidMetadata("correlationid"))?
                    .to_string()
                    .parse::<Uuid>()?,
            ),
            causation_id: CausationId::from(MessageId::from(
                event
                    .extensions()
                    .get(&"causationid".parse()?)
                    .ok_or(EventCloudEventCodecError::InvalidMetadata("causationid"))?
                    .to_string()
                    .parse::<Uuid>()?,
            )),
        };
        if event
            .partition_key()
            .as_ref()
            .map(CloudEventPartitionKey::as_str)
            != Some((format!("{}:{}", envelope.aggregate_type, envelope.aggregate_id)).as_str())
        {
            return Err(EventCloudEventCodecError::InvalidMetadata("partitionkey"));
        }
        Ok(envelope)
    }

    pub fn encode_id(event_id: EventId) -> Result<CloudEventId, EventCloudEventCodecError> {
        Ok(event_id.to_string().parse()?)
    }

    pub fn decode_id(event_id: &CloudEventId) -> Result<EventId, EventCloudEventCodecError> {
        Ok(EventId::try_from(event_id.as_str().parse::<Uuid>()?)?)
    }

    pub fn encode_type(
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
        aggregate_type: &AggregateTypeOwned,
        event_name: &EventNameOwned,
    ) -> Result<CloudEventType, EventCloudEventCodecError> {
        Ok(CloudEventType::with_prefix(
            cloud_event_type_prefix,
            &format!("{aggregate_type}.{event_name}"),
        )?)
    }

    pub fn decode_type(
        event_type: &CloudEventType,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<(AggregateTypeOwned, EventNameOwned), EventCloudEventCodecError> {
        let name = event_type.without_prefix(cloud_event_type_prefix)?;
        let (aggregate_type, event_name) = name
            .split_once('.')
            .ok_or(EventCloudEventCodecError::InvalidMetadata("type"))?;
        Ok((aggregate_type.parse()?, event_name.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_round_trip_validates_domain_id() {
        let event_id = EventId::new();
        let encoded = EventCloudEventCodec::encode_id(event_id).unwrap();
        assert_eq!(EventCloudEventCodec::decode_id(&encoded).unwrap(), event_id);
        for invalid in ["not-a-uuid", "00000000-0000-0000-0000-000000000000"] {
            assert!(EventCloudEventCodec::decode_id(&invalid.parse().unwrap()).is_err());
        }
    }

    #[test]
    fn type_round_trip_with_and_without_prefix() {
        let aggregate_type = "bank_account".parse().unwrap();
        let event_name = "created_v1".parse().unwrap();
        let prefix = "com.example.events".parse().unwrap();
        for cloud_event_type_prefix in [None, Some(&prefix)] {
            let encoded = EventCloudEventCodec::encode_type(
                cloud_event_type_prefix,
                &aggregate_type,
                &event_name,
            )
            .unwrap();
            assert_eq!(
                EventCloudEventCodec::decode_type(&encoded, cloud_event_type_prefix).unwrap(),
                (aggregate_type.clone(), event_name.clone()),
            );
        }
    }

    #[test]
    fn decode_type_rejects_invalid_components_and_prefix() {
        for invalid in [
            "account",
            ".created",
            "account.",
            "account.created.extra",
            "Account.created",
        ] {
            assert!(EventCloudEventCodec::decode_type(&invalid.parse().unwrap(), None).is_err());
        }
        assert!(
            EventCloudEventCodec::decode_type(
                &"com.other.account.created".parse().unwrap(),
                Some(&"com.example".parse().unwrap()),
            )
            .is_err()
        );
    }
}
