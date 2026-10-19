use crate::read_model::ReadModelRef;

use super::{
    ReadModelWatchEndpointId, ReadModelWatchEndpointLeaseExpiresAt, ReadModelWatchGeneration,
    ReadModelWatchId, ReadModelWatchLeaseExpiresAt, ReadModelWatchRegistryError,
};

#[allow(async_fn_in_trait)]
pub trait ReadModelWatchRegistry: Send + Sync {
    /// Registers a new endpoint. An existing ID is an error, even if its lease has expired.
    async fn register_endpoint(
        &self,
        endpoint_id: ReadModelWatchEndpointId,
        lease_until: ReadModelWatchEndpointLeaseExpiresAt,
    ) -> Result<(), ReadModelWatchRegistryError>;

    /// Extends an active endpoint's lease without shortening it.
    /// Missing or expired endpoints must not be recreated or revived.
    async fn extend_endpoint_lease(
        &self,
        endpoint_id: ReadModelWatchEndpointId,
        lease_until: ReadModelWatchEndpointLeaseExpiresAt,
    ) -> Result<(), ReadModelWatchRegistryError>;

    /// Removes the endpoint and all its watches and targets. Missing endpoints are ignored.
    async fn remove_endpoint(
        &self,
        endpoint_id: ReadModelWatchEndpointId,
    ) -> Result<(), ReadModelWatchRegistryError>;

    /// Registers a new watch under an active endpoint. An existing watch ID is an error.
    async fn register_watch(
        &self,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
        lease_until: ReadModelWatchLeaseExpiresAt,
    ) -> Result<(), ReadModelWatchRegistryError>;

    /// Extends a watch's lease without shortening it, provided both leases are active
    /// and the watch belongs to the endpoint. Missing or expired registrations are errors.
    async fn extend_watch_lease(
        &self,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
        lease_until: ReadModelWatchLeaseExpiresAt,
    ) -> Result<(), ReadModelWatchRegistryError>;

    /// Removes a watch and its targets. Missing watches are ignored; a watch belonging
    /// to another endpoint must not be removed and produces an error.
    async fn remove_watch(
        &self,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
    ) -> Result<(), ReadModelWatchRegistryError>;

    /// Atomically advances the watch's generation and returns it. Previous targets remain
    /// active until completion. Requires a matching endpoint and two active leases.
    async fn begin_generation(
        &self,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
    ) -> Result<ReadModelWatchGeneration, ReadModelWatchRegistryError>;

    /// Adds targets idempotently to the latest, incomplete generation and commits before
    /// returning. Rejects stale generations, mismatched endpoints, and inactive leases.
    async fn register_targets(
        &self,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
        generation: ReadModelWatchGeneration,
        targets: &[ReadModelRef],
    ) -> Result<(), ReadModelWatchRegistryError>;

    /// Checks inclusion in the specified latest generation. Empty targets return true.
    /// Rejects stale generations, mismatched endpoints, and inactive leases even for empty targets.
    async fn contains_targets(
        &self,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
        generation: ReadModelWatchGeneration,
        targets: &[ReadModelRef],
    ) -> Result<bool, ReadModelWatchRegistryError>;

    /// Completes the latest generation and atomically removes all older targets. Repeated
    /// completion of the same latest generation is allowed. Rejects stale generations,
    /// mismatched endpoints, and inactive leases; failed operations preserve older targets.
    async fn complete_generation(
        &self,
        endpoint_id: ReadModelWatchEndpointId,
        watch_id: ReadModelWatchId,
        generation: ReadModelWatchGeneration,
    ) -> Result<(), ReadModelWatchRegistryError>;
}
