use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelWatchIndexExpiresAtError {
    #[error("watch endpoint expiration exceeds the supported timestamp range")]
    Overflow,
}
