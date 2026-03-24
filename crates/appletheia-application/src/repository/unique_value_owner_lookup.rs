use crate::unit_of_work::UnitOfWork;
use appletheia_domain::aggregate::{AggregateId, AggregateType, UniqueKey, UniqueValue};

use super::UniqueValueOwnerLookupError;

/// Resolves the owner aggregate identifier for a unique-key reservation.
#[allow(async_fn_in_trait)]
pub trait UniqueValueOwnerLookup: Send + Sync {
    type Uow: UnitOfWork;

    /// Returns the owner aggregate identifier for the given unique value.
    async fn find_owner_id<I>(
        &self,
        uow: &mut Self::Uow,
        aggregate_type: AggregateType,
        unique_key: UniqueKey,
        unique_value: &UniqueValue,
    ) -> Result<Option<I>, UniqueValueOwnerLookupError>
    where
        I: AggregateId;
}
