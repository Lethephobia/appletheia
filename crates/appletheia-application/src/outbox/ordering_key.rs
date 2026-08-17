use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::event::{AggregateIdValue, AggregateTypeOwned};
use crate::json::CanonicalJson;
use crate::read_model::SerializedPartition;
use crate::request_context::CorrelationId;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize)]
pub struct OrderingKey(String);

impl OrderingKey {
    pub fn new(value: String) -> Result<Self, OrderingKeyError> {
        if value.trim().is_empty() {
            return Err(OrderingKeyError::Empty);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for OrderingKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for OrderingKey {
    type Err = OrderingKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl<'de> Deserialize<'de> for OrderingKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        OrderingKey::new(value).map_err(serde::de::Error::custom)
    }
}

impl From<(&AggregateTypeOwned, &AggregateIdValue)> for OrderingKey {
    fn from((aggregate_type, aggregate_id): (&AggregateTypeOwned, &AggregateIdValue)) -> Self {
        Self(format!(
            "{}:{}",
            aggregate_type.value(),
            aggregate_id.value()
        ))
    }
}

impl From<CorrelationId> for OrderingKey {
    fn from(value: CorrelationId) -> Self {
        Self(value.to_string())
    }
}

impl From<CanonicalJson> for OrderingKey {
    fn from(value: CanonicalJson) -> Self {
        Self(value.into_string())
    }
}

impl From<&SerializedPartition> for OrderingKey {
    fn from(partition: &SerializedPartition) -> Self {
        Self::from(partition.canonical_json())
    }
}

#[derive(Debug, Error)]
pub enum OrderingKeyError {
    #[error("ordering key cannot be empty")]
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_non_empty() {
        let key = OrderingKey::new("abc".to_string()).unwrap();
        assert_eq!(key.as_str(), "abc");
    }

    #[test]
    fn rejects_empty() {
        let err = OrderingKey::new("".to_string()).unwrap_err();
        assert!(matches!(err, OrderingKeyError::Empty));
    }

    #[test]
    fn source_partition_builds_one_ordered_stream_key() {
        let partition = SerializedPartition::try_from(serde_json::json!({
            "fragment_name": "user",
            "key": "019feb8c-d525-7b01-91d5-018b73dad7a7"
        }))
        .expect("partition should be valid");

        let ordering_key = OrderingKey::from(&partition);

        assert_eq!(ordering_key.as_str(), partition.canonical_json().as_str());
    }

    #[test]
    fn source_partition_ordering_key_is_independent_of_object_key_order() {
        let first = SerializedPartition::try_from(serde_json::json!({
            "fragment_name": "organization",
            "key": { "organization_id": "one", "user_id": "two" },
        }))
        .expect("first partition should be valid");
        let second = SerializedPartition::try_from(serde_json::json!({
            "key": { "user_id": "two", "organization_id": "one" },
            "fragment_name": "organization",
        }))
        .expect("second partition should be valid");

        assert_eq!(OrderingKey::from(&first), OrderingKey::from(&second));
    }
}
