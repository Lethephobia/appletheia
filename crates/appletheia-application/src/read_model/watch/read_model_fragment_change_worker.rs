use super::ReadModelFragmentChangeWorkerError;

/// Consumes one fixed shard of fragment changes for watch-session fanout.
#[allow(async_fn_in_trait)]
pub trait ReadModelFragmentChangeWorker: Send {
    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&mut self);

    async fn run_forever(&mut self) -> Result<(), ReadModelFragmentChangeWorkerError>;
}
