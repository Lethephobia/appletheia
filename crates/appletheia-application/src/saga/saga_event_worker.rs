use super::{Saga, SagaEventWorkerError};

#[allow(async_fn_in_trait)]
pub trait SagaEventWorker: Send + Sync {
    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&self);

    async fn run_forever<SG: Saga>(&self, saga: &SG) -> Result<(), SagaEventWorkerError>;
}
