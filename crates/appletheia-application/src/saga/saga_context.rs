use super::{SagaContextError, SagaInstance, SagaInstanceId, SagaState, SagaStep};
use crate::command::{Command, CommandOptions};
use crate::request_context::{CausationId, CorrelationId};

/// Provides input-scoped operations on a persisted saga instance.
pub struct SagaContext<'a, S: SagaState, T: SagaStep> {
    instance: &'a mut SagaInstance<S, T>,
    causation_id: CausationId,
    step: T,
}

impl<'a, S: SagaState, T: SagaStep> SagaContext<'a, S, T> {
    /// Creates a context for a runner executing a selected route.
    pub fn new(instance: &'a mut SagaInstance<S, T>, causation_id: CausationId, step: T) -> Self {
        Self {
            instance,
            causation_id,
            step,
        }
    }

    pub fn instance_id(&self) -> SagaInstanceId {
        self.instance.saga_instance_id
    }

    pub fn correlation_id(&self) -> CorrelationId {
        self.instance.correlation_id
    }

    pub fn state(&self) -> Option<&S> {
        self.instance.state.as_ref()
    }

    pub fn set_state(&mut self, state: S) {
        self.instance.state = Some(state);
    }

    pub fn state_required(&self) -> Result<&S, SagaContextError> {
        Ok(self.instance.state_required()?)
    }

    pub fn state_required_mut(&mut self) -> Result<&mut S, SagaContextError> {
        Ok(self.instance.state_required_mut()?)
    }

    /// Enqueues a command caused by the current event or command failure.
    pub fn append_command<C: Command>(&mut self, command: &C) -> Result<(), SagaContextError> {
        Ok(self
            .instance
            .append_command(self.causation_id, self.step, command)?)
    }

    pub fn append_command_with_options<C: Command>(
        &mut self,
        command: &C,
        options: CommandOptions,
    ) -> Result<(), SagaContextError> {
        Ok(self.instance.append_command_with_options(
            self.causation_id,
            self.step,
            command,
            options,
        )?)
    }
}
