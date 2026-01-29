use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::event::{AggregateIdValue, AggregateTypeOwned};
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
}
