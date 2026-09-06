use super::{Saga, SagaEventWorkerError};

#[allow(async_fn_in_trait)]
pub trait SagaEventWorker: Send {
    type Saga: Saga;

    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&mut self);

    async fn run_forever(&mut self) -> Result<(), SagaEventWorkerError>;
}
