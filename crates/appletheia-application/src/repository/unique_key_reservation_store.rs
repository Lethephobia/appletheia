use crate::unit_of_work::UnitOfWork;
use appletheia_domain::aggregate::{AggregateId, AggregateType, UniqueEntries};

use super::UniqueKeyReservationStoreError;

/// Persists unique-key reservations within the current unit of work.
#[allow(async_fn_in_trait)]
pub trait UniqueKeyReservationStore: Send + Sync {
    type Uow: UnitOfWork;

    /// Replaces all reservations owned by the aggregate with the provided set.
    async fn replace<I>(
        &self,
        uow: &mut Self::Uow,
        aggregate_type: AggregateType,
        owner_id: I,
        unique_entries: &UniqueEntries,
    ) -> Result<(), UniqueKeyReservationStoreError>
    where
        I: AggregateId;
}
