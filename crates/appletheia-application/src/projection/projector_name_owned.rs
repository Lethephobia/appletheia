use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{ProjectorName, ProjectorNameOwnedError};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ProjectorNameOwned(String);

impl ProjectorNameOwned {
    pub fn new(value: String) -> Result<Self, ProjectorNameOwnedError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), ProjectorNameOwnedError> {
        if value.is_empty() {
            return Err(ProjectorNameOwnedError::Empty);
        }
        let len = value.len();
        if len > ProjectorName::MAX_LENGTH {
            return Err(ProjectorNameOwnedError::TooLong {
                len,
                max: ProjectorName::MAX_LENGTH,
            });
        }
        let is_snake_ascii = value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        if !is_snake_ascii {
            return Err(ProjectorNameOwnedError::InvalidFormat {
                value: value.to_owned(),
            });
        }
        Ok(())
    }
}

impl Display for ProjectorNameOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl FromStr for ProjectorNameOwned {
    type Err = ProjectorNameOwnedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::validate(s)?;
        Ok(Self(s.to_owned()))
    }
}

impl TryFrom<&str> for ProjectorNameOwned {
    type Error = ProjectorNameOwnedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for ProjectorNameOwned {
    type Error = ProjectorNameOwnedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ProjectorName> for ProjectorNameOwned {
    fn from(value: ProjectorName) -> Self {
        Self(value.value().to_owned())
    }
}

impl From<ProjectorNameOwned> for String {
    fn from(value: ProjectorNameOwned) -> Self {
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
            "a".repeat(ProjectorName::MAX_LENGTH),
        ] {
            let json = serde_json::to_string(&value).unwrap();
            let decoded: ProjectorNameOwned = serde_json::from_str(&json).unwrap();
            assert_eq!(decoded, ProjectorNameOwned::new(value).unwrap());
            assert_eq!(serde_json::to_string(&decoded).unwrap(), json);
        }
        for value in [
            "".to_owned(),
            "Name".to_owned(),
            "name-part".to_owned(),
            "name part".to_owned(),
            "名前".to_owned(),
            "a".repeat(ProjectorName::MAX_LENGTH + 1),
        ] {
            assert!(ProjectorNameOwned::new(value.clone()).is_err());
            let json = serde_json::to_string(&value).unwrap();
            assert!(
                serde_json::from_str::<ProjectorNameOwned>(&json).is_err(),
                "{value}"
            );
        }
    }
}
