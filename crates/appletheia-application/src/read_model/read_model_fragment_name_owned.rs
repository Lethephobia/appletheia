use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{ReadModelFragmentName, ReadModelFragmentNameOwnedError};

/// Owns a validated read model fragment name received from storage or the wire.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ReadModelFragmentNameOwned(String);

impl ReadModelFragmentNameOwned {
    /// Validates and owns a fragment name.
    pub fn new(value: String) -> Result<Self, ReadModelFragmentNameOwnedError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    /// Returns the fragment name.
    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), ReadModelFragmentNameOwnedError> {
        if value.is_empty() {
            return Err(ReadModelFragmentNameOwnedError::Empty);
        }
        if value.len() > ReadModelFragmentName::MAX_LENGTH {
            return Err(ReadModelFragmentNameOwnedError::TooLong);
        }
        if !value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
        {
            return Err(ReadModelFragmentNameOwnedError::InvalidFormat);
        }

        Ok(())
    }
}

impl Display for ReadModelFragmentNameOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for ReadModelFragmentNameOwned {
    type Err = ReadModelFragmentNameOwnedError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for ReadModelFragmentNameOwned {
    type Error = ReadModelFragmentNameOwnedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for ReadModelFragmentNameOwned {
    type Error = ReadModelFragmentNameOwnedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ReadModelFragmentNameOwned> for String {
    fn from(value: ReadModelFragmentNameOwned) -> Self {
        value.0
    }
}

impl From<ReadModelFragmentName> for ReadModelFragmentNameOwned {
    fn from(value: ReadModelFragmentName) -> Self {
        Self(value.value().to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_rejects_an_invalid_name() {
        let result = serde_json::from_str::<ReadModelFragmentNameOwned>("\"InvalidName\"");

        assert!(result.is_err());
    }
}
