use serde::{Deserialize, Serialize};

use crate::messaging::{OrderingKey, PublishableMessage};
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
    fn ordering_key(&self) -> OrderingKey {
        OrderingKey::from(self.correlation_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messaging::CloudEventAttributeValue;
    use crate::messaging::CommandFailureCloudEventCodec;
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
        let mut event =
            CommandFailureCloudEventCodec::encode(&envelope, &source, Some(&prefix)).unwrap();
        assert_eq!(
            event.event_type().as_str(),
            "example.command.transfer.failed"
        );
        assert_eq!(
            CommandFailureCloudEventCodec::decode(&event, Some(&prefix)).unwrap(),
            envelope
        );
        assert_eq!(
            CommandFailureCloudEventCodec::encode(&envelope, &source, Some(&prefix)).unwrap(),
            event
        );
        event
            .insert_extension(
                "causationid".parse().unwrap(),
                CloudEventAttributeValue::String((MessageId::new().to_string()).parse().unwrap()),
            )
            .unwrap();
        assert!(CommandFailureCloudEventCodec::decode(&event, Some(&prefix)).is_err());
    }
}
