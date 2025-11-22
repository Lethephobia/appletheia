use std::marker::PhantomData;

use appletheia_application::event::TryEventWriterProvider;
use appletheia_application::request_context::{RequestContext, RequestContextAccess};
use appletheia_application::snapshot::TrySnapshotWriterProvider;
use appletheia_application::unit_of_work::{
    UnitOfWork, UnitOfWorkConfig, UnitOfWorkConfigAccess, UnitOfWorkError,
};
use appletheia_domain::{Aggregate, TrySnapshotReaderProvider};
use sqlx::{PgPool, Postgres, Transaction};

use crate::postgresql::event::PgEventWriter;
use crate::postgresql::repository::PgRepository;
use crate::postgresql::snapshot::{PgSnapshotReader, PgSnapshotWriter};

#[derive(Debug)]
pub struct PgUnitOfWork<A: Aggregate> {
    pool: PgPool,
    config: UnitOfWorkConfig,
    request_context: RequestContext,
    transaction: Option<Transaction<'static, Postgres>>,
    _aggregate: PhantomData<A>,
}

impl<A: Aggregate> PgUnitOfWork<A> {
    pub fn new(pool: PgPool, config: UnitOfWorkConfig, request_context: RequestContext) -> Self {
        Self {
            pool,
            config,
            request_context,
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
        Ok(PgEventWriter::new(
            transaction,
            self.request_context.clone(),
        ))
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

impl<A: Aggregate> RequestContextAccess for PgUnitOfWork<A> {
    fn request_context(&self) -> &RequestContext {
        &self.request_context
    }
}

impl<A: Aggregate> UnitOfWorkConfigAccess for PgUnitOfWork<A> {
    fn config(&self) -> &UnitOfWorkConfig {
        &self.config
    }
}

impl<A: Aggregate> UnitOfWork<A> for PgUnitOfWork<A> {
    type Repository<'c>
        = PgRepository<'c, A>
    where
        Self: 'c;

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
