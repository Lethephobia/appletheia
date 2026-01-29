use serde::{Deserialize, Serialize};

use crate::command::{Command, CommandNameOwned};
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
}

impl CommandEnvelope {
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
