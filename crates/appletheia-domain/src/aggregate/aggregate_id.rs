use std::{error::Error, fmt::Debug, hash::Hash};

use serde::Serialize;
use serde::de::DeserializeOwned;

use uuid::Uuid;

/// Identifies an aggregate using a UUID-backed domain ID.
///
/// This trait does not impose UUID version or value constraints by itself.
/// Implementations may enforce their own validation rules in `try_from_uuid`.
pub trait AggregateId:
    Copy + Debug + Eq + Hash + Serialize + DeserializeOwned + Send + Sync + 'static
{
    type Error: Error + Send + Sync + 'static;

    /// Returns the raw `Uuid` used for persistence and external I/O.
    fn value(&self) -> Uuid;

    /// Builds the aggregate ID from a raw `Uuid`.
    ///
    /// Implementations may validate the UUID and reject values that do not
    /// satisfy domain-specific constraints.
    fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::{Deserialize, Serialize};
    use thiserror::Error;

    #[derive(Debug, Error, Eq, PartialEq)]
    enum CounterIdError {
        #[error("nil uuid is not allowed")]
        NilUuid,
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
            if value.is_nil() {
                return Err(CounterIdError::NilUuid);
            }

            Ok(Self(value))
        }
    }

    #[test]
    fn value_returns_underlying_uuid() {
        let uuid = Uuid::now_v7();
        let aggregate_id = CounterId(uuid);

        assert_eq!(aggregate_id.value(), uuid);
    }

    #[test]
    fn try_from_uuid_accepts_valid_uuid() {
        let uuid = Uuid::now_v7();
        let aggregate_id = CounterId::try_from_uuid(uuid).expect("valid uuid should be accepted");

        assert_eq!(aggregate_id.value(), uuid);
    }

    #[test]
    fn try_from_uuid_propagates_validation_error() {
        let error = CounterId::try_from_uuid(Uuid::nil()).expect_err("nil uuid should be rejected");

        assert_eq!(error, CounterIdError::NilUuid);
    }

    #[test]
    fn aggregate_id_impl_is_copy_and_eq() {
        let aggregate_id =
            CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let copied = aggregate_id;

        assert_eq!(aggregate_id, copied);
        assert_eq!(aggregate_id.value(), copied.value());
    }
}
