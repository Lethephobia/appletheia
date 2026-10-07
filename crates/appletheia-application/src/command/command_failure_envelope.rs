use serde::{Deserialize, Serialize};

use crate::messaging::{OrderingKey, PublishableMessage};
use crate::request_context::{CausationId, CorrelationId, MessageId};
use crate::saga::SagaCommandOrigin;

use super::{
    Command, CommandAttemptCount, CommandEnvelope, CommandFailedAt, CommandFailureEnvelopeError,
    CommandFailureId, CommandNameOwned, CommandTerminalReason, SerializedCommand,
};

/// Notifies an originating saga that one of its commands failed terminally.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommandFailureEnvelope {
    pub failure_id: CommandFailureId,
    pub command_message_id: MessageId,
    pub command_name: CommandNameOwned,
    pub command: SerializedCommand,
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
            command: command.command.clone(),
            origin,
            terminal_reason,
            attempt_count,
            correlation_id: command.correlation_id,
            causation_id: CausationId::from(command.message_id),
            failed_at,
        }
    }

    pub fn try_to_command<C>(&self) -> Result<C, CommandFailureEnvelopeError>
    where
        C: Command,
    {
        let expected = CommandNameOwned::from(C::NAME);
        if self.command_name != expected {
            return Err(CommandFailureEnvelopeError::CommandNameMismatch {
                expected: expected.to_string(),
                actual: self.command_name.to_string(),
            });
        }
        Ok(serde_json::from_value(self.command.value().clone())?)
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
    use crate::command::CommandName;
    use crate::saga::{SagaInstanceId, SagaName, SagaNameOwned, SerializedSagaStep};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Close {
        account_id: u32,
    }

    impl Command for Close {
        const NAME: CommandName = CommandName::new("close");
    }

    fn failure() -> CommandFailureEnvelope {
        let command = CommandEnvelope::new(
            &Close { account_id: 42 },
            CorrelationId::from(MessageId::new().value()),
            CausationId::from(MessageId::new()),
        )
        .unwrap();
        CommandFailureEnvelope::new(
            &command,
            SagaCommandOrigin {
                saga_name: SagaNameOwned::from(SagaName::new("closure")),
                saga_instance_id: SagaInstanceId::new(),
                step: SerializedSagaStep::try_from(serde_json::json!("close")).unwrap(),
            },
            CommandTerminalReason::NonRetryable,
            CommandAttemptCount::first(),
            CommandFailedAt::now(),
        )
    }

    #[test]
    fn failed_command_survives_serialization_and_typed_decoding() {
        let envelope = failure();
        let json = serde_json::to_value(&envelope).unwrap();
        let decoded: CommandFailureEnvelope = serde_json::from_value(json).unwrap();
        assert_eq!(decoded, envelope);
        assert_eq!(
            decoded.try_to_command::<Close>().unwrap(),
            Close { account_id: 42 }
        );
    }

    #[test]
    fn typed_decoding_rejects_wrong_name_and_malformed_body() {
        let mut envelope = failure();
        envelope.command_name = CommandNameOwned::from(CommandName::new("other"));
        assert!(matches!(
            envelope.try_to_command::<Close>(),
            Err(CommandFailureEnvelopeError::CommandNameMismatch { .. })
        ));
        envelope.command_name = CommandNameOwned::from(Close::NAME);
        envelope.command =
            SerializedCommand::new(serde_json::json!({"account_id": "invalid"})).unwrap();
        assert!(matches!(
            envelope.try_to_command::<Close>(),
            Err(CommandFailureEnvelopeError::Json(_))
        ));
    }
}
