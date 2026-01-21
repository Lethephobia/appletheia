use crate::event::AppEvent;
use crate::unit_of_work::UnitOfWork;

use super::{SagaDefinition, SagaRunReport, SagaRunnerError};

#[allow(async_fn_in_trait)]
pub trait SagaRunner: Send + Sync {
    type Uow: UnitOfWork;

    async fn handle_event<D: SagaDefinition>(
        &self,
        uow: &mut Self::Uow,
        saga: &D,
        event: &AppEvent,
    ) -> Result<SagaRunReport, SagaRunnerError>;
}

