use appletheia_application::repository::{
    ReferenceIndexLookup, ReferenceIndexLookupError, ReferenceIndexLookupPage,
    ReferenceIndexLookupPageSize,
};
use appletheia_domain::aggregate::{AggregateId, AggregateType, ReferenceKey};
use sqlx::{Row, postgres::PgRow};

use crate::postgresql::unit_of_work::PgUnitOfWork;

/// Looks up aggregate IDs from persisted reference indexes.
pub struct PgReferenceIndexLookup;

impl PgReferenceIndexLookup {
    pub fn new() -> Self {
        Self
    }

    fn source_id_from_row<I>(row: PgRow) -> Result<I, ReferenceIndexLookupError>
    where
        I: AggregateId,
    {
        let source_aggregate_id = row
            .try_get("source_aggregate_id")
            .map_err(|error| ReferenceIndexLookupError::Persistence(Box::new(error)))?;

        I::try_from_uuid(source_aggregate_id)
            .map_err(|error| ReferenceIndexLookupError::SourceAggregateId(Box::new(error)))
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
        after_source_aggregate_id: Option<I>,
        limit: ReferenceIndexLookupPageSize,
    ) -> Result<ReferenceIndexLookupPage<I>, ReferenceIndexLookupError>
    where
        I: AggregateId,
        T: AggregateId,
    {
        let query_limit = limit.as_i64() + 1;
        let rows = if let Some(after_source_aggregate_id) = after_source_aggregate_id {
            sqlx::query(
                r#"
                SELECT source_aggregate_id
                FROM aggregate_reference_indexes
                WHERE source_aggregate_type = $1
                  AND namespace = $2
                  AND target_aggregate_id = $3
                  AND source_aggregate_id > $4
                ORDER BY source_aggregate_id
                LIMIT $5
                "#,
            )
            .bind(source_aggregate_type.value())
            .bind(reference_key.value())
            .bind(target_aggregate_id.value())
            .bind(after_source_aggregate_id.value())
            .bind(query_limit)
            .fetch_all(uow.transaction_mut().as_mut())
            .await
        } else {
            sqlx::query(
                r#"
                SELECT source_aggregate_id
                FROM aggregate_reference_indexes
                WHERE source_aggregate_type = $1
                  AND namespace = $2
                  AND target_aggregate_id = $3
                ORDER BY source_aggregate_id
                LIMIT $4
                "#,
            )
            .bind(source_aggregate_type.value())
            .bind(reference_key.value())
            .bind(target_aggregate_id.value())
            .bind(query_limit)
            .fetch_all(uow.transaction_mut().as_mut())
            .await
        }
        .map_err(|error| ReferenceIndexLookupError::Persistence(Box::new(error)))?;

        let page_limit = limit.as_usize();
        let mut source_ids = rows
            .into_iter()
            .map(Self::source_id_from_row)
            .collect::<Result<Vec<_>, _>>()?;

        let next_cursor = if source_ids.len() > page_limit {
            source_ids.truncate(page_limit);
            source_ids.last().copied()
        } else {
            None
        };

        Ok(ReferenceIndexLookupPage::new(source_ids, next_cursor))
    }
}
