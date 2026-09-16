use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::CloudEventAttributeString;
use super::PartitionKeyError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PartitionKey(String);

impl PartitionKey {
    pub fn new(value: String) -> Result<Self, PartitionKeyError> {
        if value.is_empty() {
            return Err(PartitionKeyError::Empty);
        }
        CloudEventAttributeString::new(value.clone())?;
        Ok(Self(value))
    }

    pub(super) fn from_validated_string(value: &CloudEventAttributeString) -> Self {
        Self(value.as_str().to_owned())
    }

    pub(super) fn into_attribute_string(self) -> CloudEventAttributeString {
        CloudEventAttributeString::from_partition_key(self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for PartitionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for PartitionKey {
    type Err = PartitionKeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for PartitionKey {
    type Error = PartitionKeyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PartitionKey> for String {
    fn from(value: PartitionKey) -> Self {
        value.0
    }
}
