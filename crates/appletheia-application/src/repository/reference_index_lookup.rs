use crate::unit_of_work::UnitOfWork;
use appletheia_domain::aggregate::{AggregateId, AggregateType, ReferenceKey};

use super::{ReferenceIndexLookupError, ReferenceIndexLookupPage, ReferenceIndexLookupPageSize};

/// Looks up aggregate IDs from persisted reference indexes.
#[allow(async_fn_in_trait)]
pub trait ReferenceIndexLookup: Send + Sync {
    type Uow: UnitOfWork;

    /// Finds a page of source aggregate IDs for the given reference index lookup.
    ///
    /// Results are ordered by the raw source aggregate UUID. When `next_cursor`
    /// is present, pass it as `after_source_aggregate_id` to read the next page.
    async fn find_source_ids<I, T>(
        &self,
        uow: &mut Self::Uow,
        source_aggregate_type: AggregateType,
        reference_key: ReferenceKey,
        target_aggregate_id: T,
        after_source_aggregate_id: Option<I>,
        limit: ReferenceIndexLookupPageSize,
    ) -> Result<ReferenceIndexLookupPage<I>, ReferenceIndexLookupError>
    where
        I: AggregateId,
        T: AggregateId;
}
