use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{SagaName, SagaNameOwnedError};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
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
        let len = value.len();
        if len > SagaName::MAX_LENGTH {
            return Err(SagaNameOwnedError::TooLong {
                len,
                max: SagaName::MAX_LENGTH,
            });
        }
        let is_snake_ascii = value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        if !is_snake_ascii {
            return Err(SagaNameOwnedError::InvalidFormat {
                value: value.to_owned(),
            });
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
        Self::validate(s)?;
        Ok(Self(s.to_owned()))
    }
}

impl TryFrom<&str> for SagaNameOwned {
    type Error = SagaNameOwnedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for SagaNameOwned {
    type Error = SagaNameOwnedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<SagaName> for SagaNameOwned {
    fn from(value: SagaName) -> Self {
        Self(value.value().to_owned())
    }
}

impl From<SagaNameOwned> for String {
    fn from(value: SagaNameOwned) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialization_enforces_name_constraints() {
        for value in [
            "_name".to_owned(),
            "name__2".to_owned(),
            "a".repeat(SagaName::MAX_LENGTH),
        ] {
            let json = serde_json::to_string(&value).unwrap();
            let decoded: SagaNameOwned = serde_json::from_str(&json).unwrap();
            assert_eq!(decoded, SagaNameOwned::new(value).unwrap());
            assert_eq!(serde_json::to_string(&decoded).unwrap(), json);
        }
        for value in [
            "".to_owned(),
            "Name".to_owned(),
            "name-part".to_owned(),
            "name part".to_owned(),
            "名前".to_owned(),
            "a".repeat(SagaName::MAX_LENGTH + 1),
        ] {
            assert!(SagaNameOwned::new(value.clone()).is_err());
            let json = serde_json::to_string(&value).unwrap();
            assert!(
                serde_json::from_str::<SagaNameOwned>(&json).is_err(),
                "{value}"
            );
        }
    }
}
