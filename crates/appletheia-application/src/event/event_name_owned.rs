use std::{fmt, fmt::Display, str::FromStr};

use appletheia_domain::EventName;
use serde::{Deserialize, Serialize};

use super::EventNameOwnedError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventNameOwned(String);

impl EventNameOwned {
    pub fn new(value: String) -> Result<Self, EventNameOwnedError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn validate(value: &str) -> Result<(), EventNameOwnedError> {
        if value.is_empty() {
            return Err(EventNameOwnedError::Empty);
        }
        if value.len() > EventName::MAX_LENGTH {
            return Err(EventNameOwnedError::TooLong);
        }
        if !value
            .as_bytes()
            .iter()
            .all(|&b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        {
            return Err(EventNameOwnedError::InvalidFormat);
        }
        Ok(())
    }
}

impl Display for EventNameOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl FromStr for EventNameOwned {
    type Err = EventNameOwnedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl TryFrom<&str> for EventNameOwned {
    type Error = EventNameOwnedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for EventNameOwned {
    type Error = EventNameOwnedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<EventName> for EventNameOwned {
    fn from(value: EventName) -> Self {
        Self(value.value().to_string())
    }
}
