use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgReadModelWatchRegistryError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error("watch endpoint is missing or expired")]
    EndpointInactive,

    #[error("watch was not found")]
    WatchNotFound,

    #[error("watch belongs to another endpoint")]
    EndpointMismatch,

    #[error("watch generation is stale")]
    StaleGeneration,
}
