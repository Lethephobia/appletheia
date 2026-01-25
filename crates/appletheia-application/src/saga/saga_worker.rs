use super::{SagaDefinition, SagaWorkerError};

#[allow(async_fn_in_trait)]
pub trait SagaWorker: Send {
    type Saga: SagaDefinition;

    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&mut self);

    async fn run_forever(&mut self) -> Result<(), SagaWorkerError>;
}
