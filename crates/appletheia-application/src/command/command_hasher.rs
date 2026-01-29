use thiserror::Error;

use crate::command::{Command, CommandHash, CommandHashError};

#[derive(Debug, Error)]
pub enum CommandHasherError {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("command hash error: {0}")]
    Hash(#[from] CommandHashError),
}

pub trait CommandHasher: Send + Sync {
    fn command_hash<C: Command>(&self, command: &C) -> Result<CommandHash, CommandHasherError>;
}
