use uuid::Uuid;

use crate::command::{CommandEnvelope, CommandNameOwned, SerializedCommand};
use crate::messaging::{
    CloudEvent, CloudEventAttributeValue, CloudEventData, CloudEventDataContentType, CloudEventId,
    CloudEventPartitionKey, CloudEventSource, CloudEventType, CloudEventTypePrefix,
};
use crate::request_context::{CausationId, CorrelationId, MessageId};
use crate::saga::SagaCommandOrigin;

use super::CommandCloudEventCodecError;

pub struct CommandCloudEventCodec;

impl CommandCloudEventCodec {
    pub fn encode(
        envelope: &CommandEnvelope,
        source: &CloudEventSource,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CloudEvent, CommandCloudEventCodecError> {
        let mut event = CloudEvent::new(
            Self::encode_id(envelope.message_id)?,
            source.clone(),
            Self::encode_type(cloud_event_type_prefix, &envelope.command_name)?,
        )
        .with_partition_key(CloudEventPartitionKey::new(
            envelope.correlation_id.to_string(),
        )?)
        .try_with_data(
            Some(CloudEventData::Json(envelope.command.value().clone())),
            Some(CloudEventDataContentType::json()),
        )?;
        for (name, value) in [
            ("correlationid", envelope.correlation_id.to_string()),
            ("causationid", envelope.causation_id.to_string()),
            ("options", serde_json::to_string(&envelope.options)?),
        ] {
            event.insert_extension(
                name.parse()?,
                CloudEventAttributeValue::String(value.parse()?),
            )?;
        }
        if let Some(origin) = &envelope.saga_origin {
            event.insert_extension(
                "sagaorigin".parse()?,
                CloudEventAttributeValue::String(serde_json::to_string(origin)?.parse()?),
            )?;
            event.insert_extension(
                "saganame".parse()?,
                CloudEventAttributeValue::String(origin.saga_name.to_string().parse()?),
            )?;
        }
        Ok(event)
    }

    pub fn decode(
        event: &CloudEvent,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CommandEnvelope, CommandCloudEventCodecError> {
        if !event
            .data_content_type()
            .is_some_and(CloudEventDataContentType::is_json)
        {
            return Err(CommandCloudEventCodecError::InvalidMetadata(
                "datacontenttype",
            ));
        }
        let data = match event.data() {
            Some(CloudEventData::Json(value)) => value.clone(),
            Some(CloudEventData::Binary(bytes)) => serde_json::from_slice(bytes)?,
            Some(CloudEventData::Text(text)) => serde_json::from_str(text)?,
            None => return Err(CommandCloudEventCodecError::InvalidMetadata("data")),
        };
        let command_name = Self::decode_type(event.event_type(), cloud_event_type_prefix)?;
        let saga_origin: Option<SagaCommandOrigin> = event
            .extensions()
            .get(&"sagaorigin".parse()?)
            .map(ToString::to_string)
            .map(|value| serde_json::from_str(&value))
            .transpose()?;
        if event
            .extensions()
            .get(&"saganame".parse()?)
            .map(ToString::to_string)
            != saga_origin
                .as_ref()
                .map(|origin| origin.saga_name.to_string())
        {
            return Err(CommandCloudEventCodecError::InvalidMetadata(
                "saganame/sagaorigin",
            ));
        }
        let envelope = CommandEnvelope {
            command_name,
            command: SerializedCommand::new(data)?,
            message_id: Self::decode_id(event.id())?,
            options: serde_json::from_str(
                &event
                    .extensions()
                    .get(&"options".parse()?)
                    .ok_or(CommandCloudEventCodecError::InvalidMetadata("options"))?
                    .to_string(),
            )?,
            saga_origin,
            correlation_id: CorrelationId::from(
                event
                    .extensions()
                    .get(&"correlationid".parse()?)
                    .ok_or(CommandCloudEventCodecError::InvalidMetadata(
                        "correlationid",
                    ))?
                    .to_string()
                    .parse::<Uuid>()?,
            ),
            causation_id: CausationId::from(MessageId::from(
                event
                    .extensions()
                    .get(&"causationid".parse()?)
                    .ok_or(CommandCloudEventCodecError::InvalidMetadata("causationid"))?
                    .to_string()
                    .parse::<Uuid>()?,
            )),
        };
        if event
            .partition_key()
            .as_ref()
            .map(CloudEventPartitionKey::as_str)
            != Some((envelope.correlation_id.to_string()).as_str())
        {
            return Err(CommandCloudEventCodecError::InvalidMetadata("partitionkey"));
        }
        Ok(envelope)
    }

    pub fn encode_id(message_id: MessageId) -> Result<CloudEventId, CommandCloudEventCodecError> {
        Ok(message_id.to_string().parse()?)
    }

    pub fn decode_id(
        cloud_event_id: &CloudEventId,
    ) -> Result<MessageId, CommandCloudEventCodecError> {
        Ok(MessageId::from(cloud_event_id.as_str().parse::<Uuid>()?))
    }

    pub fn encode_type(
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
        command_name: &CommandNameOwned,
    ) -> Result<CloudEventType, CommandCloudEventCodecError> {
        Ok(CloudEventType::with_prefix(
            cloud_event_type_prefix,
            command_name.value(),
        )?)
    }

    pub fn decode_type(
        event_type: &CloudEventType,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CommandNameOwned, CommandCloudEventCodecError> {
        Ok(event_type
            .without_prefix(cloud_event_type_prefix)?
            .parse()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_and_type_conversions_preserve_values_and_reject_invalid_metadata() {
        let id = MessageId::new();
        let encoded_id = CommandCloudEventCodec::encode_id(id).unwrap();
        assert_eq!(CommandCloudEventCodec::decode_id(&encoded_id).unwrap(), id);
        assert!(CommandCloudEventCodec::decode_id(&"invalid".parse().unwrap()).is_err());
        let prefix = "com.example".parse().unwrap();
        for cloud_event_type_prefix in [None, Some(&prefix)] {
            let command_name = "transfer".parse().unwrap();
            let encoded =
                CommandCloudEventCodec::encode_type(cloud_event_type_prefix, &command_name)
                    .unwrap();
            assert_eq!(
                CommandCloudEventCodec::decode_type(&encoded, cloud_event_type_prefix).unwrap(),
                command_name
            );
        }
        assert!(CommandCloudEventCodec::decode_type(&"wrong.type".parse().unwrap(), None).is_err());
        assert!(
            CommandCloudEventCodec::decode_type(&"wrong.type".parse().unwrap(), Some(&prefix))
                .is_err()
        );
    }
}
