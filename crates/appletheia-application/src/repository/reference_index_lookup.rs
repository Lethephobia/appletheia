use crate::unit_of_work::UnitOfWork;
use appletheia_domain::aggregate::{AggregateId, AggregateType, ReferenceKey};

use super::ReferenceIndexLookupError;

/// Looks up aggregate IDs from persisted reference indexes.
#[allow(async_fn_in_trait)]
pub trait ReferenceIndexLookup: Send + Sync {
    type Uow: UnitOfWork;

    /// Finds source aggregate IDs for the given source type, reference key, and target ID.
    async fn find_source_ids<I, T>(
        &self,
        uow: &mut Self::Uow,
        source_aggregate_type: AggregateType,
        reference_key: ReferenceKey,
        target_aggregate_id: T,
    ) -> Result<Vec<I>, ReferenceIndexLookupError>
    where
        I: AggregateId,
        T: AggregateId;
}
