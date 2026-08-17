use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{ReadModelPartName, ReadModelPartNameOwnedError};

/// Owns a validated read model part name received from storage or the wire.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ReadModelPartNameOwned(String);

impl ReadModelPartNameOwned {
    /// Validates and owns a part name.
    pub fn new(value: String) -> Result<Self, ReadModelPartNameOwnedError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    /// Returns the part name.
    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), ReadModelPartNameOwnedError> {
        if value.is_empty() {
            return Err(ReadModelPartNameOwnedError::Empty);
        }
        if value.len() > ReadModelPartName::MAX_LENGTH {
            return Err(ReadModelPartNameOwnedError::TooLong);
        }
        if !value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
        {
            return Err(ReadModelPartNameOwnedError::InvalidFormat);
        }

        Ok(())
    }
}

impl Display for ReadModelPartNameOwned {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.value())
    }
}

impl FromStr for ReadModelPartNameOwned {
    type Err = ReadModelPartNameOwnedError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for ReadModelPartNameOwned {
    type Error = ReadModelPartNameOwnedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for ReadModelPartNameOwned {
    type Error = ReadModelPartNameOwnedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ReadModelPartNameOwned> for String {
    fn from(value: ReadModelPartNameOwned) -> Self {
        value.0
    }
}

impl From<ReadModelPartName> for ReadModelPartNameOwned {
    fn from(value: ReadModelPartName) -> Self {
        Self(value.value().to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::ReadModelPartNameOwned;

    #[test]
    fn deserialize_rejects_an_invalid_name() {
        let result = serde_json::from_str::<ReadModelPartNameOwned>("\"InvalidName\"");

        assert!(result.is_err());
    }
}
