use crate::aggregate::{AggregateIdValue, AggregateTypeOwned};
use crate::json::CanonicalJson;
use crate::projection::ProjectorNameOwned;
use crate::read_model::SerializedPartition;
use crate::request_context::CorrelationId;
use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::CloudEventAttributeString;
use super::CloudEventPartitionKeyError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CloudEventPartitionKey(String);

impl CloudEventPartitionKey {
    pub fn new(value: String) -> Result<Self, CloudEventPartitionKeyError> {
        if value.is_empty() {
            return Err(CloudEventPartitionKeyError::Empty);
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

impl Display for CloudEventPartitionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CloudEventPartitionKey {
    type Err = CloudEventPartitionKeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for CloudEventPartitionKey {
    type Error = CloudEventPartitionKeyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CloudEventPartitionKey> for String {
    fn from(value: CloudEventPartitionKey) -> Self {
        value.0
    }
}

impl From<(&AggregateTypeOwned, &AggregateIdValue)> for CloudEventPartitionKey {
    fn from((aggregate_type, aggregate_id): (&AggregateTypeOwned, &AggregateIdValue)) -> Self {
        Self(format!(
            "{}:{}",
            aggregate_type.value(),
            aggregate_id.value()
        ))
    }
}

impl From<&ProjectorNameOwned> for CloudEventPartitionKey {
    fn from(projector_name: &ProjectorNameOwned) -> Self {
        Self(projector_name.value().to_owned())
    }
}

impl From<CorrelationId> for CloudEventPartitionKey {
    fn from(value: CorrelationId) -> Self {
        Self(value.to_string())
    }
}

impl From<CanonicalJson> for CloudEventPartitionKey {
    fn from(value: CanonicalJson) -> Self {
        Self(value.into_string())
    }
}

impl From<&SerializedPartition> for CloudEventPartitionKey {
    fn from(partition: &SerializedPartition) -> Self {
        Self::from(partition.canonical_json())
    }
}
