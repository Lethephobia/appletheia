use crate::unit_of_work::UnitOfWork;

use super::{SagaDefinition, SagaWorkerError};

#[allow(async_fn_in_trait)]
pub trait SagaWorker: Send {
    type Uow: UnitOfWork;

    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&mut self);

    async fn run_forever<D: SagaDefinition>(
        &mut self,
        uow: &mut Self::Uow,
        saga: &D,
    ) -> Result<(), SagaWorkerError>;
}

