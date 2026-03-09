use std::{error::Error, fmt::Debug, hash::Hash};

use serde::Serialize;

use serde::de::DeserializeOwned;

use super::AggregateId;

/// Represents the persisted state of an aggregate.
///
/// Implementations expose the aggregate identifier and provide JSON
/// conversion helpers used for serialization boundaries.
pub trait AggregateState:
    Clone + Debug + Eq + Hash + Serialize + DeserializeOwned + Send + Sync + 'static
{
    type Id: AggregateId;
    type Error: Error + From<serde_json::Error> + Send + Sync + 'static;

    /// Returns the identifier of the aggregate represented by this state.
    fn id(&self) -> Self::Id;

    /// Deserializes the state from a JSON value.
    fn try_from_json_value(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(serde_json::Error::into)
    }

    /// Serializes the state into a JSON value.
    fn into_json_value(self) -> Result<serde_json::Value, Self::Error> {
        serde_json::to_value(self).map_err(serde_json::Error::into)
    }
}

#[cfg(test)]
mod tests {
    use appletheia_macros::aggregate_id;
    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use uuid::Uuid;

    use super::AggregateState;
    use crate::aggregate::AggregateId;

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

    #[aggregate_id(error = CounterIdError, validate = validate_counter_id)]
    struct CounterId(Uuid);

    #[derive(Debug, Error)]
    enum CounterStateError {
        #[error(transparent)]
        Serde(#[from] serde_json::Error),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct CounterState {
        id: CounterId,
        count: i32,
    }

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

        assert!(matches!(error, CounterStateError::Serde(_)));
    }

    #[test]
    fn into_json_value_serializes_state() {
        let id = CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let state = CounterState { id, count: 8 };

        let value = state.into_json_value().expect("state should serialize");

        assert_eq!(value["id"], serde_json::json!(id.value()));
        assert_eq!(value["count"], serde_json::json!(8));
    }
}
