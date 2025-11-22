use std::marker::PhantomData;

use appletheia_application::snapshot::SnapshotWriter;
use appletheia_application::unit_of_work::UnitOfWorkError;
use appletheia_domain::{Aggregate, AggregateId, Snapshot};
use sqlx::{Postgres, Transaction};

pub struct PgSnapshotWriter<'c, A: Aggregate> {
    transaction: &'c mut Transaction<'static, Postgres>,
    _aggregate: PhantomData<A>,
}

impl<'c, A: Aggregate> PgSnapshotWriter<'c, A> {
    pub fn new(transaction: &'c mut Transaction<'static, Postgres>) -> Self {
        Self {
            transaction,
            _aggregate: PhantomData,
        }
    }
}

impl<'c, A: Aggregate> SnapshotWriter<A> for PgSnapshotWriter<'c, A> {
    type Error = UnitOfWorkError<A>;

    async fn write_snapshot(&mut self, snapshot: &Snapshot<A::State>) -> Result<(), Self::Error> {
        let snapshot_id = snapshot.id().value();
        let state = serde_json::to_value(snapshot.state()).map_err(UnitOfWorkError::Json)?;
        let materialized_at = snapshot.materialized_at().value();
        let aggregate_id = snapshot.aggregate_id().value();
        let aggregate_version = snapshot.aggregate_version().value();

        sqlx::query(
            r#"
            INSERT INTO snapshots (
                id, aggregate_type, aggregate_id, aggregate_version, state, materialized_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(snapshot_id)
        .bind(A::AGGREGATE_TYPE.value())
        .bind(aggregate_id)
        .bind(aggregate_version)
        .bind(state)
        .bind(materialized_at)
        .execute(self.transaction.as_mut())
        .await
        .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;

        Ok(())
    }
}
