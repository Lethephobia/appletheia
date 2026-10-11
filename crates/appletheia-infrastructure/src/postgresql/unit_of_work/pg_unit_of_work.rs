use appletheia_application::{aggregate::AggregateRef, repository::AggregateLockMode};
use std::collections::HashMap;

use appletheia_application::unit_of_work::{UnitOfWork, UnitOfWorkError};
use sqlx::{Postgres, Transaction};

pub struct PgUnitOfWork {
    transaction: Transaction<'static, Postgres>,
    aggregate_locks: HashMap<AggregateRef, AggregateLockMode>,
}

impl PgUnitOfWork {
    pub(super) fn new(transaction: Transaction<'static, Postgres>) -> Self {
        Self {
            transaction,
            aggregate_locks: HashMap::new(),
        }
    }

    pub(crate) fn aggregate_lock_mode(
        &self,
        aggregate: &AggregateRef,
    ) -> Option<AggregateLockMode> {
        self.aggregate_locks.get(aggregate).copied()
    }

    pub(crate) fn record_aggregate_lock(
        &mut self,
        aggregate: AggregateRef,
        mode: AggregateLockMode,
    ) {
        self.aggregate_locks.insert(aggregate, mode);
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
