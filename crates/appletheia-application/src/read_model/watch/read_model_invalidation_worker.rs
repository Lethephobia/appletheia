use super::ReadModelInvalidationWorkerError;

/// Consumes one fixed transport shard of read-model invalidations.
#[allow(async_fn_in_trait)]
pub trait ReadModelInvalidationWorker: Send {
    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&mut self);

    async fn run_forever(&mut self) -> Result<(), ReadModelInvalidationWorkerError>;
}
