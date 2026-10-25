use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgReadModelWatchEndpointRegistryError {
    #[error("watch endpoint is missing or expired")]
    EndpointInactive,
}
