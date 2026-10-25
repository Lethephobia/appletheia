use super::{ReadModelType, ReadModelTypeOwnedError};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ReadModelTypeOwned(String);

impl ReadModelTypeOwned {
    pub fn new(value: String) -> Result<Self, ReadModelTypeOwnedError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), ReadModelTypeOwnedError> {
        if value.is_empty() {
            return Err(ReadModelTypeOwnedError::Empty);
        }
        let len = value.len();
        if len > ReadModelType::MAX_LENGTH {
            return Err(ReadModelTypeOwnedError::TooLong {
                len,
                max: ReadModelType::MAX_LENGTH,
            });
        }
        let is_snake_ascii = value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        if !is_snake_ascii {
            return Err(ReadModelTypeOwnedError::InvalidFormat {
                value: value.to_owned(),
            });
        }
        Ok(())
    }
}

impl From<ReadModelType> for ReadModelTypeOwned {
    fn from(value: ReadModelType) -> Self {
        Self(value.value().to_owned())
    }
}

impl TryFrom<String> for ReadModelTypeOwned {
    type Error = ReadModelTypeOwnedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for ReadModelTypeOwned {
    type Error = ReadModelTypeOwnedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl FromStr for ReadModelTypeOwned {
    type Err = ReadModelTypeOwnedError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::validate(value)?;
        Ok(Self(value.to_owned()))
    }
}

impl From<ReadModelTypeOwned> for String {
    fn from(value: ReadModelTypeOwned) -> Self {
        value.0
    }
}

impl AsRef<str> for ReadModelTypeOwned {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl fmt::Display for ReadModelTypeOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_and_owned_types_accept_the_same_identifier_forms() {
        for value in [
            "user_profiles",
            "_users",
            "users__2",
            "123",
            "_",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ] {
            let static_type = ReadModelType::new(value);
            let owned = ReadModelTypeOwned::new(value.to_owned()).unwrap();
            assert_eq!(ReadModelTypeOwned::from(static_type), owned);
            assert_eq!(value.parse::<ReadModelTypeOwned>().unwrap(), owned);
            assert_eq!(ReadModelTypeOwned::try_from(value).unwrap(), owned);
            let json = serde_json::to_string(&owned).unwrap();
            assert_eq!(
                serde_json::from_str::<ReadModelTypeOwned>(&json).unwrap(),
                owned
            );
        }
    }

    #[test]
    fn static_and_owned_types_reject_invalid_identifiers() {
        for value in [
            "",
            "Users",
            "user-profiles",
            "user profiles",
            "users.profile",
            "ユーザー",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ] {
            assert!(std::panic::catch_unwind(|| ReadModelType::new(value)).is_err());
            assert!(ReadModelTypeOwned::new(value.to_owned()).is_err());
            assert!(value.parse::<ReadModelTypeOwned>().is_err());
            let json = serde_json::to_string(value).unwrap();
            assert!(serde_json::from_str::<ReadModelTypeOwned>(&json).is_err());
        }
        assert!(matches!(
            ReadModelTypeOwned::new(String::new()),
            Err(ReadModelTypeOwnedError::Empty)
        ));
        assert!(matches!(
            ReadModelTypeOwned::new("a".repeat(101)),
            Err(ReadModelTypeOwnedError::TooLong { len: 101, max: 100 })
        ));
        assert!(matches!(
            ReadModelTypeOwned::new("Users".to_owned()),
            Err(ReadModelTypeOwnedError::InvalidFormat { .. })
        ));
    }
}
