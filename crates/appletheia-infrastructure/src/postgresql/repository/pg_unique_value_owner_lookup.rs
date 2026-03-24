use appletheia_application::repository::{UniqueValueOwnerLookup, UniqueValueOwnerLookupError};
use appletheia_domain::aggregate::{AggregateId, AggregateType, UniqueKey, UniqueValue};
use sqlx::Row;

use crate::postgresql::unit_of_work::PgUnitOfWork;

/// Looks up aggregate owners from persisted unique-key reservations.
pub struct PgUniqueValueOwnerLookup;

impl PgUniqueValueOwnerLookup {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgUniqueValueOwnerLookup {
    fn default() -> Self {
        Self::new()
    }
}

impl UniqueValueOwnerLookup for PgUniqueValueOwnerLookup {
    type Uow = PgUnitOfWork;

    async fn find_owner_id<I>(
        &self,
        uow: &mut Self::Uow,
        aggregate_type: AggregateType,
        unique_key: UniqueKey,
        unique_value: &UniqueValue,
    ) -> Result<Option<I>, UniqueValueOwnerLookupError>
    where
        I: AggregateId,
    {
        let row = sqlx::query(
            r#"
            SELECT owner_aggregate_id
            FROM unique_key_reservations
            WHERE aggregate_type = $1
              AND namespace = $2
              AND normalized_value = $3
            LIMIT 1
            "#,
        )
        .bind(aggregate_type.value())
        .bind(unique_key.value())
        .bind(unique_value.normalized_key())
        .fetch_optional(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| UniqueValueOwnerLookupError::Persistence(Box::new(error)))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let owner_aggregate_id = row
            .try_get("owner_aggregate_id")
            .map_err(|error| UniqueValueOwnerLookupError::Persistence(Box::new(error)))?;

        I::try_from_uuid(owner_aggregate_id)
            .map(Some)
            .map_err(|error| UniqueValueOwnerLookupError::OwnerAggregateId(Box::new(error)))
    }
}
