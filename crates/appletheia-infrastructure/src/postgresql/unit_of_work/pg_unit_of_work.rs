use std::marker::PhantomData;

use appletheia_application::event::TryEventWriterProvider;
use appletheia_application::snapshot::TrySnapshotWriterProvider;
use appletheia_application::unit_of_work::{UnitOfWork, UnitOfWorkConfig, UnitOfWorkError};
use appletheia_domain::{Aggregate, TrySnapshotReaderProvider};
use sqlx::{PgPool, Postgres, Transaction};

use crate::postgresql::event::PgEventWriter;
use crate::postgresql::repository::PgRepository;
use crate::postgresql::snapshot::{PgSnapshotReader, PgSnapshotWriter};

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

impl<A: Aggregate> TrySnapshotWriterProvider<A> for PgUnitOfWork<A> {
    type Error = UnitOfWorkError<A>;
    type SnapshotWriter<'c>
        = PgSnapshotWriter<'c, A>
    where
        Self: 'c;

    fn try_snapshot_writer(&mut self) -> Result<Self::SnapshotWriter<'_>, Self::Error> {
        let transaction = self
            .transaction
            .as_mut()
            .ok_or(UnitOfWorkError::NotInTransaction)?;
        Ok(PgSnapshotWriter::new(transaction))
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
}
