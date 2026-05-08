use crate::unit_of_work::UnitOfWork;
use appletheia_domain::aggregate::{AggregateId, AggregateType, ReferenceEntries};

use super::ReferenceIndexStoreError;

/// Persists reference indexes within the current unit of work.
#[allow(async_fn_in_trait)]
pub trait ReferenceIndexStore: Send + Sync {
    type Uow: UnitOfWork;

    /// Replaces all reference indexes owned by the aggregate with the provided set.
    async fn replace<I>(
        &self,
        uow: &mut Self::Uow,
        aggregate_type: AggregateType,
        source_aggregate_id: I,
        reference_entries: &ReferenceEntries,
    ) -> Result<(), ReferenceIndexStoreError>
    where
        I: AggregateId;
}
