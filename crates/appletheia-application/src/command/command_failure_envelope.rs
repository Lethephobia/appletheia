use serde::{Deserialize, Serialize};

use crate::messaging::{
    CloudEvent, CloudEventSource, CloudEventTypePrefix, CommandFailureCloudEventCodec,
    PublishableMessage,
};
use crate::request_context::{CausationId, CorrelationId, MessageId};
use crate::saga::SagaCommandOrigin;

use super::{
    CommandAttemptCount, CommandEnvelope, CommandFailedAt, CommandFailureEnvelopeError,
    CommandFailureId, CommandNameOwned, CommandTerminalReason,
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
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CloudEvent, Self::Error> {
        Ok(CommandFailureCloudEventCodec::encode(
            self,
            source,
            cloud_event_type_prefix,
        )?)
    }

    fn try_from_cloud_event(
        event: &CloudEvent,
        cloud_event_type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<Self, Self::Error> {
        Ok(CommandFailureCloudEventCodec::decode(
            event,
            cloud_event_type_prefix,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messaging::CloudEventAttributeValue;
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
