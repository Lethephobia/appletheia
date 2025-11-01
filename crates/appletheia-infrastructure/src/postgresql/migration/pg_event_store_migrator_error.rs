use sqlx::migrate::MigrateError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgEventStoreMigratorError {
    #[error("migrate error: {0}")]
    Migrate(#[from] MigrateError),
}
