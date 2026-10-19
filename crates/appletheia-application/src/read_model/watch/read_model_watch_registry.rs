use crate::read_model::ReadModelRef;
use crate::unit_of_work::UnitOfWork;

use super::{
    ReadModelWatchEndpointId, ReadModelWatchGeneration, ReadModelWatchId,
    ReadModelWatchLeaseExpiresAt, ReadModelWatchRegistryError,
};

/// Performs database operations within the caller-owned unit of work.
/// The caller is responsible for committing or rolling back.
#[allow(async_fn_in_trait)]
pub trait ReadModelWatchRegistry: Send + Sync {
    type Uow: UnitOfWork;

    /// Registers a new watch under an active endpoint. An existing watch ID is an error.
    async fn register_watch(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
        lease_until: ReadModelWatchLeaseExpiresAt,
    ) -> Result<(), ReadModelWatchRegistryError>;

    /// Extends a watch's lease without shortening it, provided both leases are active
    /// and the watch belongs to the endpoint. Missing or expired registrations are errors.
    async fn extend_watch_lease(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
        lease_until: ReadModelWatchLeaseExpiresAt,
    ) -> Result<(), ReadModelWatchRegistryError>;

    /// Removes a watch and its targets. Missing watches are ignored; a watch belonging
    /// to another endpoint must not be removed and produces an error.
    async fn remove_watch(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
    ) -> Result<(), ReadModelWatchRegistryError>;

    /// Atomically advances the watch's generation and returns it. Previous targets remain
    /// active until explicitly deleted. Requires a matching endpoint and two active leases.
    async fn begin_generation(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
    ) -> Result<ReadModelWatchGeneration, ReadModelWatchRegistryError>;

    /// Adds targets idempotently to the latest generation within the supplied unit of work.
    /// The caller commits before querying under the new coverage. Rejects stale generations,
    /// mismatched endpoints, and inactive leases.
    async fn register_targets(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
        generation: ReadModelWatchGeneration,
        targets: &[ReadModelRef],
    ) -> Result<(), ReadModelWatchRegistryError>;

    /// Checks inclusion in the specified latest generation. Empty targets return true.
    /// Rejects stale generations, mismatched endpoints, and inactive leases even for empty targets.
    async fn contains_targets(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
        generation: ReadModelWatchGeneration,
        targets: &[ReadModelRef],
    ) -> Result<bool, ReadModelWatchRegistryError>;

    /// Atomically deletes targets older than the specified latest generation, without
    /// recording completion or preventing further registration in that generation.
    /// Repeated calls are allowed. Rejects stale generations, mismatched endpoints,
    /// and inactive leases; failed operations preserve older targets.
    async fn delete_previous_generation_targets(
        &self,
        uow: &mut Self::Uow,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
        generation: ReadModelWatchGeneration,
    ) -> Result<(), ReadModelWatchRegistryError>;
}
