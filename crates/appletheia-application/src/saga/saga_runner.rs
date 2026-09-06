use crate::command::CommandFailureEnvelope;
use crate::event::EventEnvelope;

use super::{Saga, SagaCommandFailureRunReport, SagaEventRunReport, SagaRunnerError};

#[allow(async_fn_in_trait)]
pub trait SagaRunner: Send + Sync {
    async fn handle_event<SG: Saga>(
        &self,
        saga: &SG,
        event: &EventEnvelope,
    ) -> Result<SagaEventRunReport, SagaRunnerError>;

    async fn handle_command_failure<SG: Saga>(
        &self,
        saga: &SG,
        failure: &CommandFailureEnvelope,
    ) -> Result<SagaCommandFailureRunReport, SagaRunnerError>;
}
