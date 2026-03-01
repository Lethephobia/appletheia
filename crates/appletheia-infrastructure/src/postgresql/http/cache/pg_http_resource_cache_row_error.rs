use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgHttpResourceCacheRowError {
    #[error("invalid persisted url")]
    InvalidUrl,
}
