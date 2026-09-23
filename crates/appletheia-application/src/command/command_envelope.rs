use crate::messaging::{CloudEvent, CloudEventSource, CloudEventTypePrefix};
use crate::messaging::{
    CloudEventAttributeValue, CloudEventData, CloudEventDataContentType, CloudEventPartitionKey,
    CloudEventType,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::messaging::PublishableMessage;
use crate::request_context::{CausationId, CorrelationId, MessageId};
use crate::saga::SagaCommandOrigin;

use super::{Command, CommandEnvelopeError, CommandNameOwned, CommandOptions, SerializedCommand};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommandEnvelope {
    pub command_name: CommandNameOwned,
    pub command: SerializedCommand,
    pub correlation_id: CorrelationId,
    pub message_id: MessageId,
    pub causation_id: CausationId,
    pub saga_origin: Option<SagaCommandOrigin>,
    pub options: CommandOptions,
}

impl CommandEnvelope {
    pub fn new<C: Command>(
        command: &C,
        correlation_id: CorrelationId,
        causation_id: CausationId,
        options: CommandOptions,
    ) -> Result<Self, CommandEnvelopeError> {
        Ok(Self {
            command_name: CommandNameOwned::from(C::NAME),
            command: SerializedCommand::new(serde_json::to_value(command)?)?,
            correlation_id,
            message_id: MessageId::new(),
            causation_id,
            saga_origin: None,
            options,
        })
    }

    /// Attaches the saga step that dispatched this command.
    pub fn with_saga_origin(mut self, saga_origin: SagaCommandOrigin) -> Self {
        self.saga_origin = Some(saga_origin);
        self
    }

    pub fn try_into_command<C>(&self) -> Result<C, CommandEnvelopeError>
    where
        C: Command,
    {
        let expected = CommandNameOwned::from(C::NAME);
        if self.command_name != expected {
            return Err(CommandEnvelopeError::CommandNameMismatch {
                expected: expected.to_string(),
                actual: self.command_name.to_string(),
            });
        }

        let json = self.command.value().clone();
        Ok(serde_json::from_value(json)?)
    }
}

impl PublishableMessage for CommandEnvelope {
    type Error = CommandEnvelopeError;

    fn try_to_cloud_event(
        &self,
        source: &CloudEventSource,
        type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CloudEvent, Self::Error> {
        let mut event = CloudEvent::new(
            self.message_id.to_string().parse()?,
            source.clone(),
            CloudEventType::with_prefix(type_prefix, self.command_name.value())?,
        )
        .with_partition_key(CloudEventPartitionKey::new(
            self.correlation_id.to_string(),
        )?);
        event.replace_data(
            Some(CloudEventData::Json(self.command.value().clone())),
            Some(CloudEventDataContentType::json()),
        )?;
        for (name, value) in [
            ("correlationid", self.correlation_id.to_string()),
            ("causationid", self.causation_id.to_string()),
            ("options", serde_json::to_string(&self.options)?),
        ] {
            event.insert_extension(
                name.parse()?,
                CloudEventAttributeValue::String(value.parse()?),
            )?;
        }
        if let Some(origin) = &self.saga_origin {
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

    fn try_from_cloud_event(
        event: &CloudEvent,
        type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<Self, Self::Error> {
        if !event
            .data_content_type()
            .is_some_and(CloudEventDataContentType::is_json)
        {
            return Err(CommandEnvelopeError::InvalidMetadata("datacontenttype"));
        }
        let data = match event.data() {
            Some(CloudEventData::Json(value)) => value.clone(),
            Some(CloudEventData::Binary(bytes)) => serde_json::from_slice(bytes)?,
            Some(CloudEventData::Text(text)) => serde_json::from_str(text)?,
            None => return Err(CommandEnvelopeError::InvalidMetadata("data")),
        };
        let name = event.event_type().without_prefix(type_prefix)?;
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
            return Err(CommandEnvelopeError::InvalidMetadata("saganame/sagaorigin"));
        }
        let envelope = Self {
            command_name: CommandNameOwned::new(name.to_owned())?,
            command: SerializedCommand::new(data)?,
            message_id: MessageId::from(event.id().as_str().parse::<Uuid>()?),
            options: serde_json::from_str(
                &event
                    .extensions()
                    .get(&"options".parse()?)
                    .ok_or(CommandEnvelopeError::InvalidMetadata("options"))?
                    .to_string(),
            )?,
            saga_origin,
            correlation_id: CorrelationId::from(
                event
                    .extensions()
                    .get(&"correlationid".parse()?)
                    .ok_or(CommandEnvelopeError::InvalidMetadata("correlationid"))?
                    .to_string()
                    .parse::<Uuid>()?,
            ),
            causation_id: CausationId::from(MessageId::from(
                event
                    .extensions()
                    .get(&"causationid".parse()?)
                    .ok_or(CommandEnvelopeError::InvalidMetadata("causationid"))?
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
            return Err(CommandEnvelopeError::InvalidMetadata("partitionkey"));
        }
        Ok(envelope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::saga::{SagaInstanceId, SagaNameOwned, SerializedSagaStep};

    #[test]
    fn cloud_event_round_trip_preserves_optional_origin_and_checks_filter_metadata() {
        let message_id = MessageId::new();
        let mut envelope = CommandEnvelope {
            command_name: CommandNameOwned::new("transfer".to_owned()).unwrap(),
            command: SerializedCommand::new(serde_json::json!({"amount": 10})).unwrap(),
            message_id,
            correlation_id: CorrelationId::from(message_id.value()),
            causation_id: CausationId::from(message_id),
            options: CommandOptions::default(),
            saga_origin: None,
        };
        let source = "urn:banking:commands".parse().unwrap();
        let event = envelope.try_to_cloud_event(&source, None).unwrap();
        assert_eq!(event.event_type().as_str(), "transfer");
        assert!(event.subject().is_none());
        assert!(event.time().is_none());
        assert_eq!(
            CommandEnvelope::try_from_cloud_event(&event, None).unwrap(),
            envelope
        );
        envelope.saga_origin = Some(SagaCommandOrigin {
            saga_name: SagaNameOwned::new("transfer".to_owned()).unwrap(),
            saga_instance_id: SagaInstanceId::new(),
            step: SerializedSagaStep::try_from(serde_json::json!("debit")).unwrap(),
        });
        let mut with_origin = envelope.try_to_cloud_event(&source, None).unwrap();
        assert_eq!(
            CommandEnvelope::try_from_cloud_event(&with_origin, None).unwrap(),
            envelope
        );
        with_origin
            .insert_extension(
                "saganame".parse().unwrap(),
                CloudEventAttributeValue::String(("other".to_owned()).parse().unwrap()),
            )
            .unwrap();
        assert!(CommandEnvelope::try_from_cloud_event(&with_origin, None).is_err());
    }
}
