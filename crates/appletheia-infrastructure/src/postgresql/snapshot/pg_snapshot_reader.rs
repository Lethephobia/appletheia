use std::marker::PhantomData;

use sqlx::{Postgres, QueryBuilder};

use appletheia_application::snapshot::{SnapshotReader, SnapshotReaderError};
use appletheia_application::unit_of_work::UnitOfWorkError;
use appletheia_domain::{Aggregate, AggregateId, AggregateVersion, Snapshot};

use crate::postgresql::snapshot::PgSnapshotRow;
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgSnapshotReader<A: Aggregate> {
    _phantom: PhantomData<A>,
}

impl<A: Aggregate> PgSnapshotReader<A> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<A: Aggregate> Default for PgSnapshotReader<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Aggregate> SnapshotReader<A> for PgSnapshotReader<A> {
    type Uow = PgUnitOfWork;

    async fn read_latest_snapshot(
        &self,
        uow: &mut Self::Uow,
        aggregate_id: A::Id,
        as_of: Option<AggregateVersion>,
    ) -> Result<Option<Snapshot<A::State>>, SnapshotReaderError> {
        let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            SELECT id, aggregate_type, aggregate_id, aggregate_version, state, materialized_at
            FROM snapshots WHERE aggregate_type = "#,
        );
        query
            .push_bind(A::TYPE.to_string())
            .push(" AND aggregate_id = ")
            .push_bind(aggregate_id.value());

        if let Some(version) = as_of {
            query
                .push(" AND aggregate_version <= ")
                .push_bind(version.value());
        }
        query.push(" ORDER BY aggregate_version DESC LIMIT 1");

        let transaction = uow.transaction_mut().map_err(|e| match e {
            UnitOfWorkError::NotInTransaction => SnapshotReaderError::NotInTransaction,
            other => SnapshotReaderError::Persistence(Box::new(other)),
        })?;

        let snapshot_row = query
            .build_query_as::<PgSnapshotRow>()
            .fetch_optional(transaction.as_mut())
            .await
            .map_err(|e| SnapshotReaderError::Persistence(Box::new(e)))?;
        let snapshot = snapshot_row
            .map(|row| row.try_into_snapshot::<A>())
            .transpose()
            .map_err(|e| SnapshotReaderError::MappingFailed(Box::new(e)))?;
        Ok(snapshot)
    }
}
