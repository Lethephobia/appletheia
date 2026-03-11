use thiserror::Error;

use crate::command::CommandHashError;

#[derive(Debug, Error)]
pub enum CommandHasherError {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("command hash error: {0}")]
    Hash(#[from] CommandHashError),
}
