use std::{fmt::Debug, hash::Hash};

use serde::Serialize;

use serde::de::DeserializeOwned;

use super::{AggregateStateError, ReferenceIndexes, UniqueConstraints};

/// Represents the persisted state of an aggregate.
///
/// Implementations define their unique-key constraints and provide JSON
/// conversion helpers used for serialization boundaries.
pub trait AggregateState:
    UniqueConstraints<Self::Error>
    + ReferenceIndexes<Self::Error>
    + Clone
    + Debug
    + Eq
    + Hash
    + Serialize
    + DeserializeOwned
    + Send
    + Sync
    + 'static
{
    type Error: std::error::Error + From<AggregateStateError> + Send + Sync + 'static;

    /// Deserializes the state from a JSON value.
    fn try_from_json_value(value: serde_json::Value) -> Result<Self, Self::Error> {
        Ok(serde_json::from_value(value).map_err(AggregateStateError::from)?)
    }

    /// Serializes the state into a JSON value.
    fn into_json_value(self) -> Result<serde_json::Value, Self::Error> {
        Ok(serde_json::to_value(self).map_err(AggregateStateError::from)?)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use uuid::Uuid;

    use super::AggregateState;
    use crate::aggregate::{
        AggregateStateError, ReferenceIndexes, UniqueConstraints, UniqueValuesError,
    };

    #[derive(Debug, Error)]
    enum CounterStateError {
        #[error(transparent)]
        AggregateState(#[from] AggregateStateError),

        #[error(transparent)]
        UniqueValues(#[from] UniqueValuesError),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct CounterState {
        count: i32,
    }

    impl UniqueConstraints<CounterStateError> for CounterState {}
    impl ReferenceIndexes<CounterStateError> for CounterState {}

    impl AggregateState for CounterState {
        type Error = CounterStateError;
    }

    #[test]
    fn try_from_json_value_deserializes_state() {
        let value = serde_json::json!({
            "count": 5
        });

        let state =
            CounterState::try_from_json_value(value).expect("json value should deserialize");

        assert_eq!(state.count, 5);
    }

    #[test]
    fn try_from_json_value_propagates_serde_errors() {
        let value = serde_json::json!({
            "count": "invalid"
        });

        let error = CounterState::try_from_json_value(value).expect_err("invalid json should fail");

        assert!(matches!(error, CounterStateError::AggregateState(_)));
    }

    #[test]
    fn into_json_value_serializes_state() {
        let state = CounterState { count: 8 };

        let value = state.into_json_value().expect("state should serialize");

        assert_eq!(value["count"], serde_json::json!(8));
    }

    #[test]
    fn unique_keys_defaults_to_empty() {
        let state = CounterState { count: 1 };

        let unique_keys = state
            .unique_entries(Uuid::now_v7())
            .expect("unique entries should build");

        assert!(unique_keys.is_empty());
    }
}
