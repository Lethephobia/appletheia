use std::str::FromStr;

use appletheia_domain::AggregateState;
use serde::{Deserialize, Serialize};

use super::SerializedAggregateStateError;

/// Serialized state content, independent of aggregate identity and version.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SerializedAggregateState(serde_json::Value);

impl SerializedAggregateState {
    pub fn value(&self) -> &serde_json::Value {
        &self.0
    }

    pub fn try_from_state<S>(state: &S) -> Result<Self, SerializedAggregateStateError>
    where
        S: AggregateState,
    {
        state
            .clone()
            .try_into_json_value()
            .map(Self)
            .map_err(|error| SerializedAggregateStateError::AggregateState(Box::new(error)))
    }

    pub fn try_to_state<S>(&self) -> Result<S, SerializedAggregateStateError>
    where
        S: AggregateState,
    {
        S::try_from_json_value(self.0.clone())
            .map_err(|error| SerializedAggregateStateError::AggregateState(Box::new(error)))
    }
}

impl From<serde_json::Value> for SerializedAggregateState {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}

impl FromStr for SerializedAggregateState {
    type Err = SerializedAggregateStateError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(serde_json::from_str(value)?))
    }
}

impl TryFrom<&str> for SerializedAggregateState {
    type Error = SerializedAggregateStateError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}
