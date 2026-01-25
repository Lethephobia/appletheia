use appletheia_application::unit_of_work::{UnitOfWork, UnitOfWorkError};
use sqlx::{Postgres, Transaction};

pub struct PgUnitOfWork {
    transaction: Transaction<'static, Postgres>,
}

impl PgUnitOfWork {
    pub(super) fn new(transaction: Transaction<'static, Postgres>) -> Self {
        Self { transaction }
    }

    pub fn transaction_mut(&mut self) -> &mut Transaction<'static, Postgres> {
        &mut self.transaction
    }
}

impl UnitOfWork for PgUnitOfWork {
    async fn commit(self) -> Result<(), UnitOfWorkError> {
        self.transaction
            .commit()
            .await
            .map_err(|e| UnitOfWorkError::CommitFailed(Box::new(e)))?;
        Ok(())
    }

    async fn rollback(self) -> Result<(), UnitOfWorkError> {
        self.transaction
            .rollback()
            .await
            .map_err(|e| UnitOfWorkError::RollbackFailed(Box::new(e)))?;
        Ok(())
    }
}
