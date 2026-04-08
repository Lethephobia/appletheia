use serde::{Deserialize, Serialize};

use crate::command::{Command, CommandNameOwned, CommandOptions};
use crate::request_context::{CausationId, CorrelationId, MessageId};

use super::CommandEnvelopeError;
use super::SerializedCommand;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommandEnvelope {
    pub command_name: CommandNameOwned,
    pub command: SerializedCommand,
    pub correlation_id: CorrelationId,
    pub message_id: MessageId,
    pub causation_id: CausationId,
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
            options,
        })
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
