use super::CommandFailureEnvelopeError;
use crate::messaging::{CloudEvent, CloudEventSource, CloudEventTime, CloudEventTypePrefix};
use crate::messaging::{
    CloudEventAttributeValue, CloudEventData, CloudEventDataContentType, CloudEventPartitionKey,
    CloudEventType,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::messaging::PublishableMessage;
use crate::request_context::{CausationId, CorrelationId, MessageId};
use crate::saga::SagaCommandOrigin;

use super::{
    CommandAttemptCount, CommandEnvelope, CommandFailedAt, CommandFailureId, CommandNameOwned,
    CommandTerminalReason,
};

/// Notifies an originating saga that one of its commands failed terminally.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommandFailureEnvelope {
    pub failure_id: CommandFailureId,
    pub command_message_id: MessageId,
    pub command_name: CommandNameOwned,
    pub origin: SagaCommandOrigin,
    pub terminal_reason: CommandTerminalReason,
    pub attempt_count: CommandAttemptCount,
    pub correlation_id: CorrelationId,
    pub causation_id: CausationId,
    pub failed_at: CommandFailedAt,
}

impl CommandFailureEnvelope {
    pub fn new(
        command: &CommandEnvelope,
        origin: SagaCommandOrigin,
        terminal_reason: CommandTerminalReason,
        attempt_count: CommandAttemptCount,
        failed_at: CommandFailedAt,
    ) -> Self {
        Self {
            failure_id: CommandFailureId::new(),
            command_message_id: command.message_id,
            command_name: command.command_name.clone(),
            origin,
            terminal_reason,
            attempt_count,
            correlation_id: command.correlation_id,
            causation_id: CausationId::from(command.message_id),
            failed_at,
        }
    }
}

impl PublishableMessage for CommandFailureEnvelope {
    type Error = CommandFailureEnvelopeError;

    fn try_to_cloud_event(
        &self,
        source: &CloudEventSource,
        type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CloudEvent, Self::Error> {
        if self.command_message_id.value() != self.causation_id.value() {
            return Err(CommandFailureEnvelopeError::InvalidMetadata(
                "command_message_id/causationid",
            ));
        }
        let mut event = CloudEvent::new(
            self.failure_id.to_string().parse()?,
            source.clone(),
            CloudEventType::with_prefix(type_prefix, &format!("{}.failed", self.command_name))?,
        )
        .with_partition_key(CloudEventPartitionKey::new(
            self.correlation_id.to_string(),
        )?);
        event.replace_data(Some(CloudEventData::Json(serde_json::json!({"terminal_reason": self.terminal_reason, "attempt_count": self.attempt_count}))), Some(CloudEventDataContentType::json()))?;
        event = event.with_time(CloudEventTime::new(self.failed_at.into())?);
        for (name, value) in [
            ("correlationid", self.correlation_id.to_string()),
            ("causationid", self.causation_id.to_string()),
            ("commandmessageid", self.command_message_id.to_string()),
            ("sagaorigin", serde_json::to_string(&self.origin)?),
            ("saganame", self.origin.saga_name.to_string()),
        ] {
            event.insert_extension(
                name.parse()?,
                CloudEventAttributeValue::String(value.parse()?),
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
            return Err(CommandFailureEnvelopeError::InvalidMetadata(
                "datacontenttype",
            ));
        }
        let data = match event.data() {
            Some(CloudEventData::Json(value)) => value.clone(),
            Some(CloudEventData::Binary(bytes)) => serde_json::from_slice(bytes)?,
            Some(CloudEventData::Text(text)) => serde_json::from_str(text)?,
            None => return Err(CommandFailureEnvelopeError::InvalidMetadata("data")),
        };
        let name = event.event_type().without_prefix(type_prefix)?;
        let command_name = name
            .strip_suffix(".failed")
            .filter(|name| !name.is_empty())
            .ok_or(CommandFailureEnvelopeError::InvalidMetadata("type"))?;
        let origin: SagaCommandOrigin = serde_json::from_str(
            &event
                .extensions()
                .get(&"sagaorigin".parse()?)
                .ok_or(CommandFailureEnvelopeError::InvalidMetadata("sagaorigin"))?
                .to_string(),
        )?;
        if origin.saga_name.value()
            != event
                .extensions()
                .get(&"saganame".parse()?)
                .ok_or(CommandFailureEnvelopeError::InvalidMetadata("saganame"))?
                .to_string()
        {
            return Err(CommandFailureEnvelopeError::InvalidMetadata(
                "saganame/sagaorigin",
            ));
        }
        let envelope = Self {
            failure_id: CommandFailureId::try_from(event.id().as_str().parse::<Uuid>()?)?,
            command_message_id: MessageId::from(
                event
                    .extensions()
                    .get(&"commandmessageid".parse()?)
                    .ok_or(CommandFailureEnvelopeError::InvalidMetadata(
                        "commandmessageid",
                    ))?
                    .to_string()
                    .parse::<Uuid>()?,
            ),
            command_name: CommandNameOwned::new(command_name.to_owned())?,
            origin,
            terminal_reason: serde_json::from_value(data.get("terminal_reason").cloned().ok_or(
                CommandFailureEnvelopeError::InvalidMetadata("terminal_reason"),
            )?)?,
            attempt_count: CommandAttemptCount::try_from(serde_json::from_value::<i64>(
                data.get("attempt_count").cloned().ok_or(
                    CommandFailureEnvelopeError::InvalidMetadata("attempt_count"),
                )?,
            )?)?,
            failed_at: event
                .time()
                .ok_or(CommandFailureEnvelopeError::InvalidMetadata("time"))?
                .value()
                .into(),
            correlation_id: CorrelationId::from(
                event
                    .extensions()
                    .get(&"correlationid".parse()?)
                    .ok_or(CommandFailureEnvelopeError::InvalidMetadata(
                        "correlationid",
                    ))?
                    .to_string()
                    .parse::<Uuid>()?,
            ),
            causation_id: CausationId::from(MessageId::from(
                event
                    .extensions()
                    .get(&"causationid".parse()?)
                    .ok_or(CommandFailureEnvelopeError::InvalidMetadata("causationid"))?
                    .to_string()
                    .parse::<Uuid>()?,
            )),
        };
        if envelope.command_message_id.value() != envelope.causation_id.value() {
            return Err(CommandFailureEnvelopeError::InvalidMetadata(
                "command_message_id/causationid",
            ));
        }
        if event
            .partition_key()
            .as_ref()
            .map(CloudEventPartitionKey::as_str)
            != Some((envelope.correlation_id.to_string()).as_str())
        {
            return Err(CommandFailureEnvelopeError::InvalidMetadata("partitionkey"));
        }
        Ok(envelope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::saga::{SagaInstanceId, SagaNameOwned, SerializedSagaStep};

    #[test]
    fn cloud_event_round_trip_preserves_failure_and_checks_causation() {
        let command_message_id = MessageId::new();
        let envelope = CommandFailureEnvelope {
            failure_id: CommandFailureId::new(),
            command_message_id,
            command_name: CommandNameOwned::new("transfer".to_owned()).unwrap(),
            origin: SagaCommandOrigin {
                saga_name: SagaNameOwned::new("transfer".to_owned()).unwrap(),
                saga_instance_id: SagaInstanceId::new(),
                step: SerializedSagaStep::try_from(serde_json::json!("debit")).unwrap(),
            },
            terminal_reason: CommandTerminalReason::RetryExhausted,
            attempt_count: CommandAttemptCount::try_from(i64::from(u32::MAX)).unwrap(),
            correlation_id: CorrelationId::from(command_message_id.value()),
            causation_id: CausationId::from(command_message_id),
            failed_at: CommandFailedAt::now(),
        };
        let source = "urn:banking:failures".parse().unwrap();
        let prefix = "example.command".parse().unwrap();
        let mut event = envelope.try_to_cloud_event(&source, Some(&prefix)).unwrap();
        assert_eq!(
            event.event_type().as_str(),
            "example.command.transfer.failed"
        );
        assert_eq!(
            CommandFailureEnvelope::try_from_cloud_event(&event, Some(&prefix)).unwrap(),
            envelope
        );
        assert_eq!(
            envelope.try_to_cloud_event(&source, Some(&prefix)).unwrap(),
            event
        );
        event
            .insert_extension(
                "causationid".parse().unwrap(),
                CloudEventAttributeValue::String((MessageId::new().to_string()).parse().unwrap()),
            )
            .unwrap();
        assert!(CommandFailureEnvelope::try_from_cloud_event(&event, Some(&prefix)).is_err());
    }
}
