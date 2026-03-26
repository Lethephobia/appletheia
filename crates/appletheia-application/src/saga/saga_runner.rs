use crate::event::EventEnvelope;

use super::{Saga, SagaRunReport, SagaRunnerError};

#[allow(async_fn_in_trait)]
pub trait SagaRunner: Send + Sync {
    async fn handle_event<SG: Saga>(
        &self,
        saga: &SG,
        event: &EventEnvelope,
    ) -> Result<SagaRunReport, SagaRunnerError>;
}
