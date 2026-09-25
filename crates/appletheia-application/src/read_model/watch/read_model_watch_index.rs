use std::collections::HashSet;

use super::{
    ReadModelWatchEndpoint, ReadModelWatchIndexError, ReadModelWatchLeaseDuration,
    ReadModelWatchSelector,
};

/// Stores routing registrations shared by watch endpoints.
#[allow(async_fn_in_trait)]
pub trait ReadModelWatchIndex: Send + Sync {
    /// Atomically replaces an endpoint's complete selector set and renews its lease.
    ///
    /// An empty set removes the endpoint. Retrying the same set is idempotent.
    /// Implementations measure the lease from their own clock and exclude expired
    /// endpoints from lookup. One registry must own each endpoint.
    /// An earlier request, including a cancelled request, must not overwrite a
    /// later replacement for that endpoint.
    async fn replace(
        &self,
        endpoint: ReadModelWatchEndpoint,
        selectors: &HashSet<ReadModelWatchSelector>,
        lease_duration: ReadModelWatchLeaseDuration,
    ) -> Result<(), ReadModelWatchIndexError>;

    /// Returns live endpoints registered for this exact selector.
    async fn find_endpoints(
        &self,
        selector: &ReadModelWatchSelector,
    ) -> Result<HashSet<ReadModelWatchEndpoint>, ReadModelWatchIndexError>;
}
