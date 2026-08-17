use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{ReadModelName, ReadModelNameOwnedError};

/// Owns a validated read model name received from storage or the wire.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ReadModelNameOwned(String);

impl ReadModelNameOwned {
    /// Validates and owns a read model name.
    pub fn new(value: String) -> Result<Self, ReadModelNameOwnedError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    /// Returns the read model name.
    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), ReadModelNameOwnedError> {
        if value.is_empty() {
            return Err(ReadModelNameOwnedError::Empty);
        }
        if value.len() > ReadModelName::MAX_LENGTH {
            return Err(ReadModelNameOwnedError::TooLong);
        }
        if !value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
        {
            return Err(ReadModelNameOwnedError::InvalidFormat);
        }
        Ok(())
    }
}

impl Display for ReadModelNameOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for ReadModelNameOwned {
    type Err = ReadModelNameOwnedError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for ReadModelNameOwned {
    type Error = ReadModelNameOwnedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for ReadModelNameOwned {
    type Error = ReadModelNameOwnedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ReadModelNameOwned> for String {
    fn from(value: ReadModelNameOwned) -> Self {
        value.0
    }
}

impl From<ReadModelName> for ReadModelNameOwned {
    fn from(value: ReadModelName) -> Self {
        Self(value.value().to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_rejects_an_invalid_name() {
        let result = serde_json::from_str::<ReadModelNameOwned>("\"InvalidName\"");

        assert!(result.is_err());
    }
}
