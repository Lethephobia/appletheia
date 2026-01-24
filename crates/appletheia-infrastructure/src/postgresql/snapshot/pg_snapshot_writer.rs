use std::marker::PhantomData;

use appletheia_application::snapshot::{SnapshotWriter, SnapshotWriterError};
use appletheia_application::unit_of_work::UnitOfWorkError;
use appletheia_domain::{Aggregate, AggregateId, Snapshot};

use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgSnapshotWriter<A: Aggregate> {
    _aggregate: PhantomData<A>,
}

impl<A: Aggregate> PgSnapshotWriter<A> {
    pub fn new() -> Self {
        Self {
            _aggregate: PhantomData,
        }
    }
}

impl<A: Aggregate> Default for PgSnapshotWriter<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Aggregate> SnapshotWriter<A> for PgSnapshotWriter<A> {
    type Uow = PgUnitOfWork;

    async fn write_snapshot(
        &self,
        uow: &mut Self::Uow,
        snapshot: &Snapshot<A::State>,
    ) -> Result<(), SnapshotWriterError> {
        let snapshot_id = snapshot.id().value();
        let state = serde_json::to_value(snapshot.state()).map_err(SnapshotWriterError::Json)?;
        let materialized_at = snapshot.materialized_at().value();
        let aggregate_id = snapshot.aggregate_id().value();
        let aggregate_version = snapshot.aggregate_version().value();

        let transaction = uow.transaction_mut().map_err(|e| match e {
            UnitOfWorkError::NotInTransaction => SnapshotWriterError::NotInTransaction,
            other => SnapshotWriterError::Persistence(Box::new(other)),
        })?;

        sqlx::query(
            r#"
            INSERT INTO snapshots (
                id, aggregate_type, aggregate_id, aggregate_version, state, materialized_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(snapshot_id)
        .bind(A::TYPE.to_string())
        .bind(aggregate_id)
        .bind(aggregate_version)
        .bind(state)
        .bind(materialized_at)
        .execute(transaction.as_mut())
        .await
        .map_err(|e| SnapshotWriterError::Persistence(Box::new(e)))?;

        Ok(())
    }
}
