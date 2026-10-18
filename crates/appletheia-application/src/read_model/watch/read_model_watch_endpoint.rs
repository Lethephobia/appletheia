use super::{
    ReadModelWatchEndpointError, ReadModelWatchEndpointId, ReadModelWatchId, ReadModelWatchReceiver,
};

#[allow(async_fn_in_trait)]
pub trait ReadModelWatchEndpoint: Send + Sync {
    type Receiver: ReadModelWatchReceiver;

    fn endpoint_id(&self) -> ReadModelWatchEndpointId;

    /// Prepares notification reception before registering the watch in the registry.
    /// A successful registration is renewed while its receiver remains alive.
    /// This method does not implicitly spawn the endpoint's processing task.
    async fn register_watch(
        &self,
        watch_id: ReadModelWatchId,
    ) -> Result<Self::Receiver, ReadModelWatchEndpointError>;

    /// Ends notification reception and lease renewal, then removes the watch from the registry.
    /// Repeated calls are allowed. Local reception and renewal stop even if removal fails.
    async fn remove_watch(
        &self,
        watch_id: ReadModelWatchId,
    ) -> Result<(), ReadModelWatchEndpointError>;

    /// Receives notifications and renews the endpoint and its active watches independently
    /// of receiver polling. The caller runs this loop explicitly.
    async fn run_forever(&self) -> Result<(), ReadModelWatchEndpointError>;
}
