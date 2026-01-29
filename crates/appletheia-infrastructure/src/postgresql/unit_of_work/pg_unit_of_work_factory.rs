use appletheia_application::unit_of_work::{UnitOfWorkFactory, UnitOfWorkFactoryError};
use sqlx::PgPool;

use super::pg_unit_of_work::PgUnitOfWork;

pub struct PgUnitOfWorkFactory {
    pool: PgPool,
}

impl PgUnitOfWorkFactory {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UnitOfWorkFactory for PgUnitOfWorkFactory {
    type Uow = PgUnitOfWork;

    async fn begin(&self) -> Result<Self::Uow, UnitOfWorkFactoryError> {
        let transaction = self
            .pool
            .begin()
            .await
            .map_err(|e| UnitOfWorkFactoryError::BeginFailed(Box::new(e)))?;
        Ok(PgUnitOfWork::new(transaction))
    }
}
