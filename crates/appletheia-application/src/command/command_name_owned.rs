use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{CommandName, CommandNameOwnedError};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CommandNameOwned(String);

impl CommandNameOwned {
    pub fn new(value: String) -> Result<Self, CommandNameOwnedError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), CommandNameOwnedError> {
        if value.is_empty() {
            return Err(CommandNameOwnedError::Empty);
        }
        if value.len() > CommandName::MAX_LENGTH {
            return Err(CommandNameOwnedError::TooLong);
        }
        if !value.as_bytes().iter().all(|&b| {
            let is_lower = b.is_ascii_lowercase();
            let is_digit = b.is_ascii_digit();
            let is_underscore = b == b'_';
            is_lower || is_digit || is_underscore
        }) {
            return Err(CommandNameOwnedError::InvalidFormat);
        }
        Ok(())
    }
}

impl Display for CommandNameOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl FromStr for CommandNameOwned {
    type Err = CommandNameOwnedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl TryFrom<&str> for CommandNameOwned {
    type Error = CommandNameOwnedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl From<CommandName> for CommandNameOwned {
    fn from(value: CommandName) -> Self {
        Self(value.value().to_string())
    }
}
