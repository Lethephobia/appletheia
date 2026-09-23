use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::CloudEventAttributeString;
use super::CloudEventTypePrefixError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CloudEventTypePrefix(String);

impl CloudEventTypePrefix {
    pub fn new(value: String) -> Result<Self, CloudEventTypePrefixError> {
        if value.is_empty() {
            return Err(CloudEventTypePrefixError::Empty);
        }
        if value.split('.').any(str::is_empty) {
            return Err(CloudEventTypePrefixError::EmptySegment);
        }
        CloudEventAttributeString::new(value.clone())?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for CloudEventTypePrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CloudEventTypePrefix {
    type Err = CloudEventTypePrefixError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for CloudEventTypePrefix {
    type Error = CloudEventTypePrefixError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CloudEventTypePrefix> for String {
    fn from(value: CloudEventTypePrefix) -> Self {
        value.0
    }
}
