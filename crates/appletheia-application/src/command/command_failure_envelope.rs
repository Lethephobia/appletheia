use serde::{Deserialize, Serialize};

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
