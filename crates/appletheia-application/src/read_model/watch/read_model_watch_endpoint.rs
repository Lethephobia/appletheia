use super::{
    ReadModelWatchEndpointError, ReadModelWatchEndpointId, ReadModelWatchId, ReadModelWatchReceiver,
};

#[allow(async_fn_in_trait)]
pub trait ReadModelWatchEndpoint: Send + Sync {
    type Receiver: ReadModelWatchReceiver;

    fn is_stop_requested(&self) -> bool;

    /// Interrupts idle waits and stops after current delivery and database operations finish.
    fn request_graceful_stop(&self);

    fn endpoint_id(&self) -> ReadModelWatchEndpointId;

    /// Prepares notification reception before registering the watch in the registry.
    /// Watch persistence and lease renewal are managed by the watch itself.
    /// This method does not implicitly spawn the endpoint's processing task.
    async fn register_watch(
        &self,
        watch_id: ReadModelWatchId,
    ) -> Result<Self::Receiver, ReadModelWatchEndpointError>;

    /// Ends local notification reception without modifying the registry.
    /// Repeated calls are allowed.
    async fn remove_watch(
        &self,
        watch_id: ReadModelWatchId,
    ) -> Result<(), ReadModelWatchEndpointError>;

    /// Receives notifications and renews the endpoint lease independently
    /// of receiver polling. The caller runs this loop explicitly.
    async fn run_forever(&self) -> Result<(), ReadModelWatchEndpointError>;
}
