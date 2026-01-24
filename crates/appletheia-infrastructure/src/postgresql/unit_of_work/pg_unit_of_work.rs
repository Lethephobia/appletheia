use appletheia_application::unit_of_work::{UnitOfWork, UnitOfWorkError};
use sqlx::{PgPool, Postgres, Transaction};

pub struct PgUnitOfWork {
    pool: PgPool,
    transaction: Option<Transaction<'static, Postgres>>,
}

impl PgUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            transaction: None,
        }
    }

    pub fn transaction_mut(
        &mut self,
    ) -> Result<&mut Transaction<'static, Postgres>, UnitOfWorkError> {
        self.transaction
            .as_mut()
            .ok_or(UnitOfWorkError::NotInTransaction)
    }
}

impl UnitOfWork for PgUnitOfWork {
    async fn begin(&mut self) -> Result<(), UnitOfWorkError> {
        if self.is_in_transaction() {
            return Err(UnitOfWorkError::AlreadyInTransaction);
        }
        self.transaction = Some(
            self.pool
                .begin()
                .await
                .map_err(|e| UnitOfWorkError::BeginFailed(Box::new(e)))?,
        );
        Ok(())
    }

    async fn commit(&mut self) -> Result<(), UnitOfWorkError> {
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

    async fn rollback(&mut self) -> Result<(), UnitOfWorkError> {
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
