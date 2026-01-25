use crate::event::EventEnvelope;

use super::{SagaDefinition, SagaRunReport, SagaRunnerError};

#[allow(async_fn_in_trait)]
pub trait SagaRunner: Send + Sync {
    async fn handle_event<D: SagaDefinition>(
        &self,
        saga: &D,
        event: &EventEnvelope,
    ) -> Result<SagaRunReport, SagaRunnerError>;
}
