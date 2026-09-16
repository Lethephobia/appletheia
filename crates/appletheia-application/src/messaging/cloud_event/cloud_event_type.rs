use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::CloudEventAttributeString;
use super::CloudEventTypeError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CloudEventType(String);

impl CloudEventType {
    pub fn new(value: String) -> Result<Self, CloudEventTypeError> {
        if value.is_empty() {
            return Err(CloudEventTypeError::Empty);
        }
        CloudEventAttributeString::new(value.clone())?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for CloudEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CloudEventType {
    type Err = CloudEventTypeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for CloudEventType {
    type Error = CloudEventTypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CloudEventType> for String {
    fn from(value: CloudEventType) -> Self {
        value.0
    }
}
