use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{SagaName, SagaNameOwnedError};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SagaNameOwned(String);

impl SagaNameOwned {
    pub fn new(value: String) -> Result<Self, SagaNameOwnedError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), SagaNameOwnedError> {
        if value.is_empty() {
            return Err(SagaNameOwnedError::Empty);
        }
        if value.len() > SagaName::MAX_LENGTH {
            return Err(SagaNameOwnedError::TooLong);
        }
        if !value.as_bytes().iter().all(|&b| {
            let is_lower = b.is_ascii_lowercase();
            let is_digit = b.is_ascii_digit();
            let is_underscore = b == b'_';
            is_lower || is_digit || is_underscore
        }) {
            return Err(SagaNameOwnedError::InvalidFormat);
        }
        Ok(())
    }
}

impl Display for SagaNameOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl FromStr for SagaNameOwned {
    type Err = SagaNameOwnedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl TryFrom<&str> for SagaNameOwned {
    type Error = SagaNameOwnedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl From<SagaName> for SagaNameOwned {
    fn from(value: SagaName) -> Self {
        Self(value.value().to_owned())
    }
}
