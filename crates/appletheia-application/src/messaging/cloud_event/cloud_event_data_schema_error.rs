use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudEventDataSchemaError {
    #[error("invalid URI syntax")]
    InvalidUri,
}
