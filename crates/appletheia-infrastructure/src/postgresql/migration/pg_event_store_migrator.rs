use crate::core::migration::EventStoreMigrator;
use sqlx::PgPool;

use super::pg_event_store_migrator_error::PgEventStoreMigratorError;

pub struct PgEventStoreMigrator {
    pool: PgPool,
}

impl PgEventStoreMigrator {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl EventStoreMigrator for PgEventStoreMigrator {
    type Error = PgEventStoreMigratorError;

    async fn run(&self) -> Result<(), Self::Error> {
        sqlx::migrate!("./migrations/postgresql")
            .run(&self.pool)
            .await?;
        Ok(())
    }
}
