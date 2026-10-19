use super::{
    ReadModelWatchEndpointId, ReadModelWatchEndpointLeaseExpiresAt,
    ReadModelWatchEndpointRegistryError,
};

#[allow(async_fn_in_trait)]
pub trait ReadModelWatchEndpointRegistry: Send + Sync {
    /// Registers a new endpoint. An existing ID is an error, even if its lease has expired.
    async fn register_endpoint(
        &self,
        endpoint_id: ReadModelWatchEndpointId,
        lease_until: ReadModelWatchEndpointLeaseExpiresAt,
    ) -> Result<(), ReadModelWatchEndpointRegistryError>;

    /// Extends an active endpoint's lease without shortening it.
    /// Missing or expired endpoints must not be recreated or revived.
    async fn extend_endpoint_lease(
        &self,
        endpoint_id: ReadModelWatchEndpointId,
        lease_until: ReadModelWatchEndpointLeaseExpiresAt,
    ) -> Result<(), ReadModelWatchEndpointRegistryError>;

    /// Removes the endpoint and all its watches and targets. Missing endpoints are ignored.
    async fn remove_endpoint(
        &self,
        endpoint_id: ReadModelWatchEndpointId,
    ) -> Result<(), ReadModelWatchEndpointRegistryError>;
}
