use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadModelResourceKeyError {
    #[error("invalid resource field name: {0}")]
    InvalidName(String),

    #[error("reserved resource field name: {0}")]
    ReservedName(String),
}
