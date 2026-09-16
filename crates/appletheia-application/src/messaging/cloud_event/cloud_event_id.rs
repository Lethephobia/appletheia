use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::CloudEventAttributeString;
use super::CloudEventIdError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CloudEventId(String);

impl CloudEventId {
    pub fn new(value: String) -> Result<Self, CloudEventIdError> {
        if value.is_empty() {
            return Err(CloudEventIdError::Empty);
        }
        CloudEventAttributeString::new(value.clone())?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for CloudEventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CloudEventId {
    type Err = CloudEventIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for CloudEventId {
    type Error = CloudEventIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CloudEventId> for String {
    fn from(value: CloudEventId) -> Self {
        value.0
    }
}
