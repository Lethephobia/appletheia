use std::{fmt::Debug, hash::Hash};

use serde::Serialize;

use serde::de::DeserializeOwned;

use super::{AggregateId, AggregateStateError, UniqueConstraints};

/// Represents the persisted state of an aggregate.
///
/// Implementations expose the aggregate identifier, define their unique-key
/// constraints, and provide JSON conversion helpers used for serialization
/// boundaries.
pub trait AggregateState:
    UniqueConstraints<Self::Error>
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
    type Id: AggregateId;
    type Error: std::error::Error + From<AggregateStateError> + Send + Sync + 'static;

    /// Returns the identifier of the aggregate represented by this state.
    fn id(&self) -> Self::Id;

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
        AggregateId, AggregateStateError, UniqueConstraints, UniqueValuesError,
    };

    #[derive(Debug, Error, Eq, PartialEq)]
    enum CounterIdError {
        #[error("nil uuid is not allowed")]
        NilUuid,
    }

    fn validate_counter_id(value: Uuid) -> Result<(), CounterIdError> {
        if value.is_nil() {
            return Err(CounterIdError::NilUuid);
        }

        Ok(())
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    struct CounterId(Uuid);

    impl AggregateId for CounterId {
        type Error = CounterIdError;

        fn value(&self) -> Uuid {
            self.0
        }

        fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
            validate_counter_id(value)?;
            Ok(Self(value))
        }
    }

    #[derive(Debug, Error)]
    enum CounterStateError {
        #[error(transparent)]
        AggregateState(#[from] AggregateStateError),

        #[error(transparent)]
        UniqueValues(#[from] UniqueValuesError),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct CounterState {
        id: CounterId,
        count: i32,
    }

    impl UniqueConstraints<CounterStateError> for CounterState {}

    impl AggregateState for CounterState {
        type Id = CounterId;
        type Error = CounterStateError;

        fn id(&self) -> Self::Id {
            self.id
        }
    }

    #[test]
    fn id_returns_underlying_aggregate_id() {
        let state = CounterState {
            id: CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted"),
            count: 3,
        };

        assert_eq!(state.id(), state.id);
    }

    #[test]
    fn try_from_json_value_deserializes_state() {
        let id = CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let value = serde_json::json!({
            "id": id.value(),
            "count": 5
        });

        let state =
            CounterState::try_from_json_value(value).expect("json value should deserialize");

        assert_eq!(state.id(), id);
        assert_eq!(state.count, 5);
    }

    #[test]
    fn try_from_json_value_propagates_serde_errors() {
        let value = serde_json::json!({
            "id": Uuid::now_v7(),
            "count": "invalid"
        });

        let error = CounterState::try_from_json_value(value).expect_err("invalid json should fail");

        assert!(matches!(error, CounterStateError::AggregateState(_)));
    }

    #[test]
    fn into_json_value_serializes_state() {
        let id = CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let state = CounterState { id, count: 8 };

        let value = state.into_json_value().expect("state should serialize");

        assert_eq!(value["id"], serde_json::json!(id.value()));
        assert_eq!(value["count"], serde_json::json!(8));
    }

    #[test]
    fn unique_keys_defaults_to_empty() {
        let state = CounterState {
            id: CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted"),
            count: 1,
        };

        let unique_keys = state.unique_entries().expect("unique entries should build");

        assert!(unique_keys.is_empty());
    }
}
