use appletheia_domain::EventId;

use crate::command::{Command, CommandEnvelope, CommandOptions};
use crate::request_context::{CausationId, CorrelationId};

use super::{
    SagaCommandOrigin, SagaDispatchedCommand, SagaInstanceError, SagaInstanceId, SagaNameOwned,
    SagaState, SagaStatus, SagaStep, SerializedSagaStep,
};

#[derive(Clone, Debug, PartialEq)]
pub struct SagaInstance<S: SagaState, T: SagaStep> {
    pub saga_instance_id: SagaInstanceId,
    pub saga_name: SagaNameOwned,
    pub correlation_id: CorrelationId,
    pub start_event_id: EventId,
    pub status: SagaStatus,
    pub state: Option<S>,
    pub dispatched_commands: Vec<SagaDispatchedCommand<T>>,
    pub uncommitted_commands: Vec<CommandEnvelope>,
}

impl<S: SagaState, T: SagaStep> SagaInstance<S, T> {
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
            dispatched_commands: Vec::new(),
            uncommitted_commands: Vec::new(),
        }
    }

    pub fn uncommitted_commands(&self) -> &[CommandEnvelope] {
        &self.uncommitted_commands
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

    /// Appends a command attributed to one stable logical saga step.
    pub fn append_command<C: Command>(
        &mut self,
        causation_id: CausationId,
        step: T,
        command: &C,
    ) -> Result<(), SagaInstanceError> {
        self.append_command_with_options(causation_id, step, command, CommandOptions::default())
    }

    /// Appends a command with explicit saga step and command options.
    pub fn append_command_with_options<C: Command>(
        &mut self,
        causation_id: CausationId,
        step: T,
        command: &C,
        options: CommandOptions,
    ) -> Result<(), SagaInstanceError> {
        let origin = SagaCommandOrigin {
            saga_name: self.saga_name.clone(),
            saga_instance_id: self.saga_instance_id,
            step: SerializedSagaStep::new(step)?,
        };
        let envelope = CommandEnvelope::new(command, self.correlation_id, causation_id, options)?
            .with_saga_origin(origin);
        self.uncommitted_commands.push(envelope);

        Ok(())
    }

    pub fn clear_uncommitted_commands(&mut self) {
        self.uncommitted_commands.clear();
    }
}
