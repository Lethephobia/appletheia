use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{CloudEventAttributeStringError, CloudEventPartitionKey};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CloudEventAttributeString(String);

impl CloudEventAttributeString {
    pub fn new(value: String) -> Result<Self, CloudEventAttributeStringError> {
        if value.chars().any(|character| {
            let code = character as u32;
            character.is_control()
                || (0xfdd0..=0xfdef).contains(&code)
                || code & 0xffff == 0xfffe
                || code & 0xffff == 0xffff
        }) {
            return Err(CloudEventAttributeStringError::InvalidCharacter);
        }
        Ok(Self(value))
    }

    pub(super) fn from_partition_key(value: CloudEventPartitionKey) -> Self {
        Self(String::from(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for CloudEventAttributeString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CloudEventAttributeString {
    type Err = CloudEventAttributeStringError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for CloudEventAttributeString {
    type Error = CloudEventAttributeStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CloudEventAttributeString> for String {
    fn from(value: CloudEventAttributeString) -> Self {
        value.0
    }
}
