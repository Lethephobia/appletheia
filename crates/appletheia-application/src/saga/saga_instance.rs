use crate::request_context::CorrelationId;
use crate::{command::Command, event::EventEnvelope};

use super::SagaState;
use super::{SagaAppendCommandError, SagaInstanceId, SagaNameOwned};
use crate::command::CommandNameOwned;
use crate::outbox::command::{CommandEnvelope, SerializedCommand};
use crate::request_context::{CausationId, MessageId};

#[derive(Clone, Debug, PartialEq)]
pub struct SagaInstance<S: SagaState> {
    pub saga_instance_id: SagaInstanceId,
    pub saga_name: SagaNameOwned,
    pub correlation_id: CorrelationId,
    pub status: SagaStatus,
    pub state: Option<S>,
    pub uncommitted_commands: Vec<CommandEnvelope>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SagaStatus {
    InProgress,
    Succeeded,
    Failed,
}

impl<S: SagaState> SagaInstance<S> {
    pub fn new(saga_name: SagaNameOwned, correlation_id: CorrelationId) -> Self {
        Self {
            saga_instance_id: SagaInstanceId::new(),
            saga_name,
            correlation_id,
            status: SagaStatus::InProgress,
            state: None,
            uncommitted_commands: Vec::new(),
        }
    }

    pub fn state_mut(&mut self) -> &mut Option<S> {
        &mut self.state
    }

    pub fn succeed(&mut self) {
        self.status = SagaStatus::Succeeded;
        self.clear_uncommitted_commands();
    }

    pub fn fail(&mut self) {
        self.status = SagaStatus::Failed;
        self.clear_uncommitted_commands();
    }

    pub fn append_command<C: Command>(
        &mut self,
        event_from: &EventEnvelope,
        command: &C,
    ) -> Result<(), SagaAppendCommandError> {
        if self.correlation_id != event_from.correlation_id {
            return Err(SagaAppendCommandError::CorrelationIdMismatch);
        }

        let command_name = CommandNameOwned::from(C::NAME);
        let json = serde_json::to_value(command)?;
        let serialized_command = SerializedCommand::new(json)?;

        self.uncommitted_commands.push(CommandEnvelope {
            command_name,
            command: serialized_command,
            correlation_id: self.correlation_id,
            message_id: MessageId::new(),
            causation_id: CausationId::from(event_from.event_id),
        });

        Ok(())
    }

    pub fn uncommitted_commands(&self) -> &[CommandEnvelope] {
        &self.uncommitted_commands
    }

    pub fn clear_uncommitted_commands(&mut self) {
        self.uncommitted_commands.clear();
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self.status, SagaStatus::Succeeded | SagaStatus::Failed)
    }

    pub fn is_succeeded(&self) -> bool {
        matches!(self.status, SagaStatus::Succeeded)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self.status, SagaStatus::Failed)
    }
}
