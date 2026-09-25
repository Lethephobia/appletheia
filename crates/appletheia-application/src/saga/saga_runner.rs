use super::{SagaState, SagaStep};
use crate::command::CommandFailureEnvelope;
use crate::event::EventEnvelope;
use std::error::Error;

use super::{SagaCommandFailureRunReport, SagaDefinition, SagaEventRunReport, SagaRunnerError};

#[allow(async_fn_in_trait)]
pub trait SagaRunner: Send + Sync {
    async fn handle_event<SS: SagaState, ST: SagaStep, SE: Error + Send + Sync + 'static>(
        &self,
        saga_definition: &SagaDefinition<'_, SS, ST, SE>,
        event: &EventEnvelope,
    ) -> Result<SagaEventRunReport, SagaRunnerError<SE>>;

    async fn handle_command_failure<
        SS: SagaState,
        ST: SagaStep,
        SE: Error + Send + Sync + 'static,
    >(
        &self,
        saga_definition: &SagaDefinition<'_, SS, ST, SE>,
        failure: &CommandFailureEnvelope,
    ) -> Result<SagaCommandFailureRunReport, SagaRunnerError<SE>>;
}
