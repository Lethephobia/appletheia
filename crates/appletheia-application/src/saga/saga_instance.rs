use appletheia_domain::EventId;

use crate::request_context::{CausationId, CorrelationId, MessageId};
use crate::{
    command::{Command, CommandEnvelope, CommandOptions},
    event::EventEnvelope,
};

use super::{SagaInstanceError, SagaInstanceId, SagaNameOwned, SagaState, SagaStatus};

#[derive(Clone, Debug, PartialEq)]
pub struct SagaInstance<S: SagaState> {
    pub saga_instance_id: SagaInstanceId,
    pub saga_name: SagaNameOwned,
    pub correlation_id: CorrelationId,
    pub start_event_id: EventId,
    pub status: SagaStatus,
    pub state: Option<S>,
    pub dispatched_command_message_ids: Vec<MessageId>,
    pub uncommitted_commands: Vec<CommandEnvelope>,
}

impl<S: SagaState> SagaInstance<S> {
    pub fn new(
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
        start_event_id: EventId,
    ) -> Self {
        Self {
            saga_instance_id: SagaInstanceId::new(),
            saga_name,
            correlation_id,
            start_event_id,
            status: SagaStatus::InProgress,
            state: None,
            dispatched_command_message_ids: Vec::new(),
            uncommitted_commands: Vec::new(),
        }
    }

    pub fn state_mut(&mut self) -> &mut Option<S> {
        &mut self.state
    }

    /// Returns the current saga state or a `NoState` error.
    pub fn state_required(&self) -> Result<&S, SagaInstanceError> {
        self.state.as_ref().ok_or(SagaInstanceError::NoState)
    }

    /// Returns the current saga state mutably or a `NoState` error.
    pub fn state_required_mut(&mut self) -> Result<&mut S, SagaInstanceError> {
        self.state.as_mut().ok_or(SagaInstanceError::NoState)
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
        from_event: &EventEnvelope,
        command: &C,
    ) -> Result<(), SagaInstanceError> {
        self.append_command_with_options(from_event, command, CommandOptions::default())
    }

    pub fn append_command_with_options<C: Command>(
        &mut self,
        from_event: &EventEnvelope,
        command: &C,
        options: CommandOptions,
    ) -> Result<(), SagaInstanceError> {
        if self.correlation_id != from_event.correlation_id {
            return Err(SagaInstanceError::CorrelationIdMismatch);
        }

        let envelope = CommandEnvelope::new(
            command,
            self.correlation_id,
            CausationId::from(from_event.event_id),
            options,
        )?;
        self.dispatched_command_message_ids
            .push(envelope.message_id);
        self.uncommitted_commands.push(envelope);

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
