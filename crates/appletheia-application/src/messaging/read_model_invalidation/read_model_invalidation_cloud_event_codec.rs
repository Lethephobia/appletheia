use appletheia_domain::EventId;
use uuid::Uuid;

use crate::event::EventSequence;
use crate::messaging::{
    CloudEvent, CloudEventAttributeValue, CloudEventData, CloudEventDataContentType, CloudEventId,
    CloudEventPartitionKey, CloudEventSource, CloudEventTime, CloudEventType, CloudEventTypePrefix,
};
use crate::projection::ProjectorNameOwned;
use crate::read_model::{ReadModelInvalidationEnvelope, ReadModelInvalidationId};
use crate::request_context::{CausationId, CorrelationId, MessageId};

use super::ReadModelInvalidationCloudEventCodecError;

pub struct ReadModelInvalidationCloudEventCodec;

impl ReadModelInvalidationCloudEventCodec {
    pub fn encode(
        envelope: &ReadModelInvalidationEnvelope,
        source: &CloudEventSource,
        type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CloudEvent, ReadModelInvalidationCloudEventCodecError> {
        if envelope.source_event_id.value() != envelope.causation_id.value() {
            return Err(ReadModelInvalidationCloudEventCodecError::InvalidMetadata(
                "source_event_id/causationid",
            ));
        }
        if envelope.invalidated_partitions.is_empty() {
            return Err(ReadModelInvalidationCloudEventCodecError::EmptyPartitions);
        }
        let mut event = CloudEvent::new(
            Self::encode_id(envelope.invalidation_id)?,
            source.clone(),
            Self::encode_type(type_prefix)?,
        )
        .with_partition_key(CloudEventPartitionKey::new(
            envelope.source_projector_name.to_string(),
        )?)
        .try_with_data(
            Some(CloudEventData::Json(
                serde_json::json!({"invalidated_partitions": envelope.invalidated_partitions}),
            )),
            Some(CloudEventDataContentType::json()),
        )?;
        for (name, value) in [
            ("correlationid", envelope.correlation_id.to_string()),
            ("causationid", envelope.causation_id.to_string()),
            ("sourceeventid", envelope.source_event_id.to_string()),
            (
                "sourceeventsequence",
                envelope.source_event_sequence.to_string(),
            ),
            (
                "sourceprojectorname",
                envelope.source_projector_name.to_string(),
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
                envelope.source_event_occurred_at.into(),
            )?),
        )?;
        Ok(event)
    }

    pub fn decode(
        event: &CloudEvent,
        type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<ReadModelInvalidationEnvelope, ReadModelInvalidationCloudEventCodecError> {
        if !event
            .data_content_type()
            .is_some_and(CloudEventDataContentType::is_json)
        {
            return Err(ReadModelInvalidationCloudEventCodecError::InvalidMetadata(
                "datacontenttype",
            ));
        }
        let data = match event.data() {
            Some(CloudEventData::Json(value)) => value.clone(),
            Some(CloudEventData::Binary(bytes)) => serde_json::from_slice(bytes)?,
            Some(CloudEventData::Text(text)) => serde_json::from_str(text)?,
            None => {
                return Err(ReadModelInvalidationCloudEventCodecError::InvalidMetadata(
                    "data",
                ));
            }
        };
        Self::decode_type(event.event_type(), type_prefix)?;
        let envelope = ReadModelInvalidationEnvelope {
            invalidation_id: Self::decode_id(event.id())?,
            source_event_id: EventId::try_from(
                event
                    .extensions()
                    .get(&"sourceeventid".parse()?)
                    .ok_or(ReadModelInvalidationCloudEventCodecError::InvalidMetadata(
                        "sourceeventid",
                    ))?
                    .to_string()
                    .parse::<Uuid>()?,
            )?,
            source_event_sequence: EventSequence::try_from(
                event
                    .extensions()
                    .get(&"sourceeventsequence".parse()?)
                    .ok_or(ReadModelInvalidationCloudEventCodecError::InvalidMetadata(
                        "sourceeventsequence",
                    ))?
                    .to_string()
                    .parse::<i64>()?,
            )?,
            source_projector_name: ProjectorNameOwned::new(
                event
                    .extensions()
                    .get(&"sourceprojectorname".parse()?)
                    .ok_or(ReadModelInvalidationCloudEventCodecError::InvalidMetadata(
                        "sourceprojectorname",
                    ))?
                    .to_string(),
            )?,
            source_event_occurred_at: event
                .extensions()
                .get(&"sourceeventoccurredat".parse()?)
                .ok_or(ReadModelInvalidationCloudEventCodecError::InvalidMetadata(
                    "sourceeventoccurredat",
                ))?
                .to_string()
                .parse::<CloudEventTime>()?
                .value()
                .into(),
            invalidated_partitions: serde_json::from_value(
                data.get("invalidated_partitions").cloned().ok_or(
                    ReadModelInvalidationCloudEventCodecError::InvalidMetadata(
                        "invalidated_partitions",
                    ),
                )?,
            )?,
            correlation_id: CorrelationId::from(
                event
                    .extensions()
                    .get(&"correlationid".parse()?)
                    .ok_or(ReadModelInvalidationCloudEventCodecError::InvalidMetadata(
                        "correlationid",
                    ))?
                    .to_string()
                    .parse::<Uuid>()?,
            ),
            causation_id: CausationId::from(MessageId::from(
                event
                    .extensions()
                    .get(&"causationid".parse()?)
                    .ok_or(ReadModelInvalidationCloudEventCodecError::InvalidMetadata(
                        "causationid",
                    ))?
                    .to_string()
                    .parse::<Uuid>()?,
            )),
        };
        if envelope.source_event_id.value() != envelope.causation_id.value() {
            return Err(ReadModelInvalidationCloudEventCodecError::InvalidMetadata(
                "source_event_id/causationid",
            ));
        }
        if envelope.invalidated_partitions.is_empty() {
            return Err(ReadModelInvalidationCloudEventCodecError::EmptyPartitions);
        }
        if event
            .partition_key()
            .as_ref()
            .map(CloudEventPartitionKey::as_str)
            != Some((envelope.source_projector_name.to_string()).as_str())
        {
            return Err(ReadModelInvalidationCloudEventCodecError::InvalidMetadata(
                "partitionkey",
            ));
        }
        Ok(envelope)
    }

    pub fn encode_id(
        invalidation_id: ReadModelInvalidationId,
    ) -> Result<CloudEventId, ReadModelInvalidationCloudEventCodecError> {
        Ok(invalidation_id.to_string().parse()?)
    }

    pub fn decode_id(
        cloud_event_id: &CloudEventId,
    ) -> Result<ReadModelInvalidationId, ReadModelInvalidationCloudEventCodecError> {
        Ok(ReadModelInvalidationId::try_from(
            cloud_event_id.as_str().parse::<Uuid>()?,
        )?)
    }

    pub fn encode_type(
        type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CloudEventType, ReadModelInvalidationCloudEventCodecError> {
        Ok(CloudEventType::with_prefix(
            type_prefix,
            "read_model.invalidated",
        )?)
    }

    pub fn decode_type(
        event_type: &CloudEventType,
        type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<(), ReadModelInvalidationCloudEventCodecError> {
        if event_type.without_prefix(type_prefix)? != "read_model.invalidated" {
            return Err(ReadModelInvalidationCloudEventCodecError::InvalidMetadata(
                "type",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_and_type_conversions_preserve_values_and_reject_invalid_metadata() {
        let id = ReadModelInvalidationId::new();
        let encoded_id = ReadModelInvalidationCloudEventCodec::encode_id(id).unwrap();
        assert_eq!(
            ReadModelInvalidationCloudEventCodec::decode_id(&encoded_id).unwrap(),
            id
        );
        assert!(
            ReadModelInvalidationCloudEventCodec::decode_id(&"invalid".parse().unwrap()).is_err()
        );
        let prefix = "com.example".parse().unwrap();
        for type_prefix in [None, Some(&prefix)] {
            let encoded = ReadModelInvalidationCloudEventCodec::encode_type(type_prefix).unwrap();
            ReadModelInvalidationCloudEventCodec::decode_type(&encoded, type_prefix).unwrap();
        }
        assert!(
            ReadModelInvalidationCloudEventCodec::decode_type(&"wrong.type".parse().unwrap(), None)
                .is_err()
        );
        assert!(
            ReadModelInvalidationCloudEventCodec::decode_type(
                &"wrong.type".parse().unwrap(),
                Some(&prefix)
            )
            .is_err()
        );
    }
}
