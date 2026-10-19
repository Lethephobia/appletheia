use crate::unit_of_work::UnitOfWork;

use super::{
    ReadModelWatchEndpointId, ReadModelWatchEndpointLeaseExpiresAt,
    ReadModelWatchEndpointRegistryError,
};

/// Performs database operations within the caller-owned unit of work.
/// The caller is responsible for committing or rolling back.
#[allow(async_fn_in_trait)]
pub trait ReadModelWatchEndpointRegistry: Send + Sync {
    type Uow: UnitOfWork;

    /// Registers a new endpoint. An existing ID is an error, even if its lease has expired.
    async fn register_endpoint(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        lease_until: ReadModelWatchEndpointLeaseExpiresAt,
    ) -> Result<(), ReadModelWatchEndpointRegistryError>;

    /// Extends an active endpoint's lease without shortening it.
    /// Missing or expired endpoints must not be recreated or revived.
    async fn extend_endpoint_lease(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        lease_until: ReadModelWatchEndpointLeaseExpiresAt,
    ) -> Result<(), ReadModelWatchEndpointRegistryError>;

    /// Removes the endpoint and all its watches and targets. Missing endpoints are ignored.
    async fn remove_endpoint(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
    ) -> Result<(), ReadModelWatchEndpointRegistryError>;
}
