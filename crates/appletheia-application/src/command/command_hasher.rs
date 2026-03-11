use crate::command::{Command, CommandHash, CommandHasherError};

pub trait CommandHasher: Send + Sync {
    fn command_hash<C: Command>(&self, command: &C) -> Result<CommandHash, CommandHasherError>;
}
