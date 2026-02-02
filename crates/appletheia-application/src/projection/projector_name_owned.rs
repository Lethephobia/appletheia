use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{ProjectorName, ProjectorNameOwnedError};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
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
        if value.len() > ProjectorName::MAX_LENGTH {
            return Err(ProjectorNameOwnedError::TooLong);
        }
        if !value.as_bytes().iter().all(|&b| {
            let is_lower = b.is_ascii_lowercase();
            let is_digit = b.is_ascii_digit();
            let is_underscore = b == b'_';
            is_lower || is_digit || is_underscore
        }) {
            return Err(ProjectorNameOwnedError::InvalidFormat);
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
        Self::new(s.to_string())
    }
}

impl TryFrom<&str> for ProjectorNameOwned {
    type Error = ProjectorNameOwnedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl From<ProjectorName> for ProjectorNameOwned {
    fn from(value: ProjectorName) -> Self {
        Self(value.value().to_owned())
    }
}
