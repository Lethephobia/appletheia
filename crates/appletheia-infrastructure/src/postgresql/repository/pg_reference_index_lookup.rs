use appletheia_application::repository::{ReferenceIndexLookup, ReferenceIndexLookupError};
use appletheia_domain::aggregate::{AggregateId, AggregateType, ReferenceKey};
use sqlx::Row;

use crate::postgresql::unit_of_work::PgUnitOfWork;

/// Looks up aggregate IDs from persisted reference indexes.
pub struct PgReferenceIndexLookup;

impl PgReferenceIndexLookup {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgReferenceIndexLookup {
    fn default() -> Self {
        Self::new()
    }
}

impl ReferenceIndexLookup for PgReferenceIndexLookup {
    type Uow = PgUnitOfWork;

    async fn find_source_ids<I, T>(
        &self,
        uow: &mut Self::Uow,
        source_aggregate_type: AggregateType,
        reference_key: ReferenceKey,
        target_aggregate_id: T,
    ) -> Result<Vec<I>, ReferenceIndexLookupError>
    where
        I: AggregateId,
        T: AggregateId,
    {
        let rows = sqlx::query(
            r#"
            SELECT source_aggregate_id
            FROM aggregate_reference_indexes
            WHERE source_aggregate_type = $1
              AND namespace = $2
              AND target_aggregate_id = $3
            ORDER BY source_aggregate_id
            "#,
        )
        .bind(source_aggregate_type.value())
        .bind(reference_key.value())
        .bind(target_aggregate_id.value())
        .fetch_all(uow.transaction_mut().as_mut())
        .await
        .map_err(|error| ReferenceIndexLookupError::Persistence(Box::new(error)))?;

        rows.into_iter()
            .map(|row| {
                let source_aggregate_id = row
                    .try_get("source_aggregate_id")
                    .map_err(|error| ReferenceIndexLookupError::Persistence(Box::new(error)))?;

                I::try_from_uuid(source_aggregate_id)
                    .map_err(|error| ReferenceIndexLookupError::SourceAggregateId(Box::new(error)))
            })
            .collect()
    }
}
