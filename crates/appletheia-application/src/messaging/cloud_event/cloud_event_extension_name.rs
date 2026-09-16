use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::CloudEventExtensionNameError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CloudEventExtensionName(String);

impl CloudEventExtensionName {
    pub fn new(value: String) -> Result<Self, CloudEventExtensionNameError> {
        if value.is_empty()
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        {
            return Err(CloudEventExtensionNameError::InvalidName);
        }
        if matches!(
            value.as_str(),
            "specversion"
                | "id"
                | "source"
                | "type"
                | "subject"
                | "time"
                | "datacontenttype"
                | "dataschema"
                | "data"
        ) {
            return Err(CloudEventExtensionNameError::ReservedName);
        }
        Ok(Self(value))
    }

    pub fn partition_key() -> Self {
        Self("partitionkey".to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for CloudEventExtensionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CloudEventExtensionName {
    type Err = CloudEventExtensionNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for CloudEventExtensionName {
    type Error = CloudEventExtensionNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CloudEventExtensionName> for String {
    fn from(value: CloudEventExtensionName) -> Self {
        value.0
    }
}
