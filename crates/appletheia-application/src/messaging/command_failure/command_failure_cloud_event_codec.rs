use uuid::Uuid;

use crate::command::{
    CommandAttemptCount, CommandFailureEnvelope, CommandFailureId, CommandNameOwned,
};
use crate::messaging::{
    CloudEvent, CloudEventAttributeValue, CloudEventData, CloudEventDataContentType, CloudEventId,
    CloudEventPartitionKey, CloudEventSource, CloudEventTime, CloudEventType, CloudEventTypePrefix,
};
use crate::request_context::{CausationId, CorrelationId, MessageId};
use crate::saga::SagaCommandOrigin;

use super::CommandFailureCloudEventCodecError;

pub struct CommandFailureCloudEventCodec;

impl CommandFailureCloudEventCodec {
    pub fn encode(
        envelope: &CommandFailureEnvelope,
        source: &CloudEventSource,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CloudEvent, CommandFailureCloudEventCodecError> {
        if envelope.command_message_id.value() != envelope.causation_id.value() {
            return Err(CommandFailureCloudEventCodecError::InvalidMetadata(
                "command_message_id/causationid",
            ));
        }
        let mut event = CloudEvent::new(
            Self::encode_id(envelope.failure_id)?,
            source.clone(),
            Self::encode_type(cloud_event_type_prefix, &envelope.command_name)?,
        )
        .with_partition_key(CloudEventPartitionKey::new(
            envelope.correlation_id.to_string(),
        )?)
        .try_with_data(Some(CloudEventData::Json(serde_json::json!({"terminal_reason": envelope.terminal_reason, "attempt_count": envelope.attempt_count}))), Some(CloudEventDataContentType::json()))?
        .with_time(CloudEventTime::new(envelope.failed_at.into())?);
        for (name, value) in [
            ("correlationid", envelope.correlation_id.to_string()),
            ("causationid", envelope.causation_id.to_string()),
            ("commandmessageid", envelope.command_message_id.to_string()),
            ("sagaorigin", serde_json::to_string(&envelope.origin)?),
            ("saganame", envelope.origin.saga_name.to_string()),
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
    ) -> Result<CommandFailureEnvelope, CommandFailureCloudEventCodecError> {
        if !event
            .data_content_type()
            .is_some_and(CloudEventDataContentType::is_json)
        {
            return Err(CommandFailureCloudEventCodecError::InvalidMetadata(
                "datacontenttype",
            ));
        }
        let data = match event.data() {
            Some(CloudEventData::Json(value)) => value.clone(),
            Some(CloudEventData::Binary(bytes)) => serde_json::from_slice(bytes)?,
            Some(CloudEventData::Text(text)) => serde_json::from_str(text)?,
            None => return Err(CommandFailureCloudEventCodecError::InvalidMetadata("data")),
        };
        let command_name = Self::decode_type(event.event_type(), cloud_event_type_prefix)?;
        let origin: SagaCommandOrigin = serde_json::from_str(
            &event
                .extensions()
                .get(&"sagaorigin".parse()?)
                .ok_or(CommandFailureCloudEventCodecError::InvalidMetadata(
                    "sagaorigin",
                ))?
                .to_string(),
        )?;
        if origin.saga_name.value()
            != event
                .extensions()
                .get(&"saganame".parse()?)
                .ok_or(CommandFailureCloudEventCodecError::InvalidMetadata(
                    "saganame",
                ))?
                .to_string()
        {
            return Err(CommandFailureCloudEventCodecError::InvalidMetadata(
                "saganame/sagaorigin",
            ));
        }
        let envelope = CommandFailureEnvelope {
            failure_id: Self::decode_id(event.id())?,
            command_message_id: MessageId::from(
                event
                    .extensions()
                    .get(&"commandmessageid".parse()?)
                    .ok_or(CommandFailureCloudEventCodecError::InvalidMetadata(
                        "commandmessageid",
                    ))?
                    .to_string()
                    .parse::<Uuid>()?,
            ),
            command_name,
            origin,
            terminal_reason: serde_json::from_value(data.get("terminal_reason").cloned().ok_or(
                CommandFailureCloudEventCodecError::InvalidMetadata("terminal_reason"),
            )?)?,
            attempt_count: CommandAttemptCount::try_from(serde_json::from_value::<i64>(
                data.get("attempt_count").cloned().ok_or(
                    CommandFailureCloudEventCodecError::InvalidMetadata("attempt_count"),
                )?,
            )?)?,
            failed_at: event
                .time()
                .ok_or(CommandFailureCloudEventCodecError::InvalidMetadata("time"))?
                .value()
                .into(),
            correlation_id: CorrelationId::from(
                event
                    .extensions()
                    .get(&"correlationid".parse()?)
                    .ok_or(CommandFailureCloudEventCodecError::InvalidMetadata(
                        "correlationid",
                    ))?
                    .to_string()
                    .parse::<Uuid>()?,
            ),
            causation_id: CausationId::from(MessageId::from(
                event
                    .extensions()
                    .get(&"causationid".parse()?)
                    .ok_or(CommandFailureCloudEventCodecError::InvalidMetadata(
                        "causationid",
                    ))?
                    .to_string()
                    .parse::<Uuid>()?,
            )),
        };
        if envelope.command_message_id.value() != envelope.causation_id.value() {
            return Err(CommandFailureCloudEventCodecError::InvalidMetadata(
                "command_message_id/causationid",
            ));
        }
        if event
            .partition_key()
            .as_ref()
            .map(CloudEventPartitionKey::as_str)
            != Some((envelope.correlation_id.to_string()).as_str())
        {
            return Err(CommandFailureCloudEventCodecError::InvalidMetadata(
                "partitionkey",
            ));
        }
        Ok(envelope)
    }

    pub fn encode_id(
        failure_id: CommandFailureId,
    ) -> Result<CloudEventId, CommandFailureCloudEventCodecError> {
        Ok(failure_id.to_string().parse()?)
    }

    pub fn decode_id(
        cloud_event_id: &CloudEventId,
    ) -> Result<CommandFailureId, CommandFailureCloudEventCodecError> {
        Ok(CommandFailureId::try_from(
            cloud_event_id.as_str().parse::<Uuid>()?,
        )?)
    }

    pub fn encode_type(
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
        command_name: &CommandNameOwned,
    ) -> Result<CloudEventType, CommandFailureCloudEventCodecError> {
        Ok(CloudEventType::with_prefix(
            cloud_event_type_prefix,
            &format!("{command_name}.failed"),
        )?)
    }

    pub fn decode_type(
        event_type: &CloudEventType,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CommandNameOwned, CommandFailureCloudEventCodecError> {
        let name = event_type.without_prefix(cloud_event_type_prefix)?;
        let command_name = name
            .strip_suffix(".failed")
            .ok_or(CommandFailureCloudEventCodecError::InvalidMetadata("type"))?;
        Ok(command_name.parse()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_and_type_conversions_preserve_values_and_reject_invalid_metadata() {
        let id = CommandFailureId::new();
        let encoded_id = CommandFailureCloudEventCodec::encode_id(id).unwrap();
        assert_eq!(
            CommandFailureCloudEventCodec::decode_id(&encoded_id).unwrap(),
            id
        );
        assert!(CommandFailureCloudEventCodec::decode_id(&"invalid".parse().unwrap()).is_err());
        let prefix = "com.example".parse().unwrap();
        for cloud_event_type_prefix in [None, Some(&prefix)] {
            let command_name = "transfer".parse().unwrap();
            let encoded =
                CommandFailureCloudEventCodec::encode_type(cloud_event_type_prefix, &command_name)
                    .unwrap();
            assert_eq!(
                CommandFailureCloudEventCodec::decode_type(&encoded, cloud_event_type_prefix)
                    .unwrap(),
                command_name
            );
        }
        assert!(
            CommandFailureCloudEventCodec::decode_type(&"wrong.type".parse().unwrap(), None)
                .is_err()
        );
        assert!(
            CommandFailureCloudEventCodec::decode_type(
                &"wrong.type".parse().unwrap(),
                Some(&prefix)
            )
            .is_err()
        );
    }
}
