use std::fmt::{self, Display};

use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CommandHash(String);

#[derive(Debug, Error)]
pub enum CommandHashError {
    #[error("command hash must be 64 lowercase hex chars")]
    InvalidFormat,
}

impl CommandHash {
    pub const LENGTH: usize = 64;

    pub fn new(value: String) -> Result<Self, CommandHashError> {
        if value.len() != Self::LENGTH {
            return Err(CommandHashError::InvalidFormat);
        }
        if !value
            .as_bytes()
            .iter()
            .all(|&b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        {
            return Err(CommandHashError::InvalidFormat);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CommandHash {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for CommandHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<CommandHash> for String {
    fn from(value: CommandHash) -> Self {
        value.0
    }
}
