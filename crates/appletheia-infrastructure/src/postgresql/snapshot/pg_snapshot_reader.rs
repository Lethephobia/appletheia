use std::marker::PhantomData;

use sqlx::{Postgres, QueryBuilder, Transaction};

use appletheia_application::snapshot::{SnapshotReader, SnapshotReaderError};
use appletheia_domain::{Aggregate, AggregateId, AggregateVersion, Snapshot};

use crate::postgresql::snapshot::PgSnapshotRow;

pub struct PgSnapshotReader<'c, A: Aggregate> {
    transaction: &'c mut Transaction<'static, Postgres>,
    _phantom: PhantomData<A>,
}

impl<'c, A: Aggregate> PgSnapshotReader<'c, A> {
    pub(crate) fn new(transaction: &'c mut Transaction<'static, Postgres>) -> Self {
        Self {
            transaction,
            _phantom: PhantomData,
        }
    }
}

impl<'c, A: Aggregate> SnapshotReader<A> for PgSnapshotReader<'c, A> {
    async fn read_latest_snapshot(
        &mut self,
        aggregate_id: A::Id,
        as_of: Option<AggregateVersion>,
    ) -> Result<Option<Snapshot<A::State>>, SnapshotReaderError> {
        let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, state, materialized_at
            FROM snapshots WHERE aggregate_type = "#,
        );
        query
            .push_bind(A::AGGREGATE_TYPE.value())
            .push(" AND aggregate_id = ")
            .push_bind(aggregate_id.value());

        if let Some(version) = as_of {
            query
                .push(" AND aggregate_version <= ")
                .push_bind(version.value());
        }
        query.push(" ORDER BY aggregate_version DESC LIMIT 1");

        let snapshot_row = query
            .build_query_as::<PgSnapshotRow>()
            .fetch_optional(self.transaction.as_mut())
            .await
            .map_err(|e| SnapshotReaderError::Persistence(Box::new(e)))?;
        let snapshot = snapshot_row
            .map(|row| row.try_into_snapshot::<A>())
            .transpose()
            .map_err(|e| SnapshotReaderError::MappingFailed(Box::new(e)))?;
        Ok(snapshot)
    }
}
