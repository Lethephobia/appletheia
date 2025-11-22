use std::marker::PhantomData;

use appletheia_application::event::TryEventWriterProvider;
use appletheia_application::unit_of_work::{UnitOfWork, UnitOfWorkConfig, UnitOfWorkError};
use appletheia_domain::{Aggregate, AggregateId, Snapshot, TrySnapshotReaderProvider};
use sqlx::{PgPool, Postgres, Transaction};

use crate::postgresql::event::PgEventWriter;
use crate::postgresql::repository::PgRepository;
use crate::postgresql::snapshot::PgSnapshotReader;

#[derive(Debug)]
pub struct PgUnitOfWork<A: Aggregate> {
    pool: PgPool,
    config: UnitOfWorkConfig,
    transaction: Option<Transaction<'static, Postgres>>,
    _aggregate: PhantomData<A>,
}

impl<A: Aggregate> PgUnitOfWork<A> {
    pub fn new(pool: PgPool, config: UnitOfWorkConfig) -> Self {
        Self {
            pool,
            config,
            transaction: None,
            _aggregate: PhantomData,
        }
    }
}

impl<A: Aggregate> TryEventWriterProvider<A> for PgUnitOfWork<A> {
    type Error = UnitOfWorkError<A>;
    type EventWriter<'c>
        = PgEventWriter<'c, A>
    where
        Self: 'c;

    fn try_event_writer(&mut self) -> Result<Self::EventWriter<'_>, Self::Error> {
        let transaction = self
            .transaction
            .as_mut()
            .ok_or(UnitOfWorkError::NotInTransaction)?;
        let request_context = self.config.request_context.clone();
        Ok(PgEventWriter::new(transaction, request_context))
    }
}

impl<A: Aggregate> TrySnapshotReaderProvider<A> for PgUnitOfWork<A> {
    type Error = UnitOfWorkError<A>;
    type SnapshotReader<'c>
        = PgSnapshotReader<'c, A>
    where
        Self: 'c;

    fn try_snapshot_reader(&mut self) -> Result<Self::SnapshotReader<'_>, Self::Error> {
        let transaction = self
            .transaction
            .as_mut()
            .ok_or(UnitOfWorkError::NotInTransaction)?;
        Ok(PgSnapshotReader::new(transaction))
    }
}

impl<A: Aggregate> UnitOfWork<A> for PgUnitOfWork<A> {
    type Repository<'c>
        = PgRepository<'c, A>
    where
        Self: 'c;

    fn config(&self) -> &UnitOfWorkConfig {
        &self.config
    }

    fn repository(&mut self) -> Result<Self::Repository<'_>, UnitOfWorkError<A>> {
        let transaction = self
            .transaction
            .as_mut()
            .ok_or(UnitOfWorkError::NotInTransaction)?;
        Ok(PgRepository::new(transaction))
    }

    async fn begin(&mut self) -> Result<(), UnitOfWorkError<A>> {
        if self.is_in_transaction() {
            return Ok(());
        }
        self.transaction = Some(
            self.pool
                .begin()
                .await
                .map_err(|e| UnitOfWorkError::BeginFailed(Box::new(e)))?,
        );
        Ok(())
    }

    async fn commit(&mut self) -> Result<(), UnitOfWorkError<A>> {
        if !self.is_in_transaction() {
            return Err(UnitOfWorkError::NotInTransaction);
        }
        self.transaction
            .take()
            .ok_or(UnitOfWorkError::NotInTransaction)?
            .commit()
            .await
            .map_err(|e| UnitOfWorkError::CommitFailed(Box::new(e)))?;
        Ok(())
    }

    async fn rollback(&mut self) -> Result<(), UnitOfWorkError<A>> {
        if !self.is_in_transaction() {
            return Err(UnitOfWorkError::NotInTransaction);
        }
        self.transaction
            .take()
            .ok_or(UnitOfWorkError::NotInTransaction)?
            .rollback()
            .await
            .map_err(|e| UnitOfWorkError::RollbackFailed(Box::new(e)))?;
        Ok(())
    }

    fn is_in_transaction(&self) -> bool {
        self.transaction.is_some()
    }

    async fn write_snapshot(
        &mut self,
        snapshot: &Snapshot<A::State>,
    ) -> Result<(), UnitOfWorkError<A>> {
        let transaction = self
            .transaction
            .as_mut()
            .ok_or(UnitOfWorkError::NotInTransaction)?;
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
        .execute(transaction.as_mut())
        .await
        .map_err(|e| UnitOfWorkError::Persistence(Box::new(e)))?;
        Ok(())
    }
}
