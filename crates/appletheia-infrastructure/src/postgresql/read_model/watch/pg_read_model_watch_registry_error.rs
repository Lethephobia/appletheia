use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgReadModelWatchRegistryError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error("watch endpoint is missing or expired")]
    EndpointInactive,

    #[error("watch is missing or expired")]
    WatchInactive,

    #[error("watch belongs to another endpoint")]
    EndpointMismatch,

    #[error("watch generation is stale")]
    StaleGeneration,
}
