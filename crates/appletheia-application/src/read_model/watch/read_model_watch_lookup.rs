use crate::read_model::ReadModelRef;

use super::{
    ReadModelWatchEndpointId, ReadModelWatchEndpointPage, ReadModelWatchEndpointPageSize,
    ReadModelWatchId, ReadModelWatchLookupError, ReadModelWatchPage, ReadModelWatchPageSize,
};

#[allow(async_fn_in_trait)]
pub trait ReadModelWatchLookup: Send + Sync {
    /// Finds distinct endpoint IDs with a matching target in any retained generation.
    /// Both endpoint and watch leases must be active. IDs are ordered by their raw UUID,
    /// strictly after `cursor`, and deduplicated before applying `limit`.
    /// When more results exist, `next_cursor` is the last returned ID; otherwise it is `None`.
    /// Each page observes current registrations and leases, not a snapshot across pages.
    async fn find_endpoint_ids_by_target(
        &self,
        target: &ReadModelRef,
        cursor: Option<ReadModelWatchEndpointId>,
        limit: ReadModelWatchEndpointPageSize,
    ) -> Result<ReadModelWatchEndpointPage, ReadModelWatchLookupError>;

    /// Finds distinct watch IDs belonging to the endpoint and matching the target in any
    /// retained generation. Both endpoint and watch leases must be active.
    /// IDs are ordered by their raw UUID, strictly after `cursor`, and deduplicated before
    /// applying `limit`. When more results exist, `next_cursor` is the last returned ID;
    /// otherwise it is `None`. Each page observes current registrations and leases.
    async fn find_watch_ids_by_target(
        &self,
        endpoint_id: ReadModelWatchEndpointId,
        target: &ReadModelRef,
        cursor: Option<ReadModelWatchId>,
        limit: ReadModelWatchPageSize,
    ) -> Result<ReadModelWatchPage, ReadModelWatchLookupError>;
}
