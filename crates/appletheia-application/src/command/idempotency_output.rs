use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::IdempotencyOutputError;

/// Stores the replay-safe command result persisted for idempotent replays.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IdempotencyOutput(serde_json::Value);

impl IdempotencyOutput {
    /// Creates an idempotency output from a serialized JSON value.
    pub fn new(value: serde_json::Value) -> Self {
        Self(value)
    }

    /// Returns the underlying serialized JSON value.
    pub fn value(&self) -> &serde_json::Value {
        &self.0
    }

    /// Consumes the wrapper and returns the underlying serialized JSON value.
    pub fn into_value(self) -> serde_json::Value {
        self.0
    }
}

impl From<serde_json::Value> for IdempotencyOutput {
    fn from(value: serde_json::Value) -> Self {
        Self::new(value)
    }
}

impl From<IdempotencyOutput> for serde_json::Value {
    fn from(value: IdempotencyOutput) -> Self {
        value.into_value()
    }
}

impl Display for IdempotencyOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for IdempotencyOutput {
    type Err = IdempotencyOutputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(serde_json::from_str(s)?))
    }
}

impl TryFrom<&str> for IdempotencyOutput {
    type Error = IdempotencyOutputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for IdempotencyOutput {
    type Error = IdempotencyOutputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let json = serde_json::from_str::<serde_json::Value>(&value)?;
        Ok(Self::new(json))
    }
}
