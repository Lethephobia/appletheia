use appletheia_domain::{Aggregate, AggregateId};
use serde::{Deserialize, Serialize};

use super::AggregateRefError;
use crate::event::{AggregateIdValue, AggregateTypeOwned};

/// References an aggregate by type and identifier.
///
/// This value is used when application-layer components need to point to an
/// aggregate without loading its full state.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AggregateRef {
    /// The stable aggregate type identifier.
    pub aggregate_type: AggregateTypeOwned,
    /// The persisted aggregate identifier value.
    pub aggregate_id: AggregateIdValue,
}

impl AggregateRef {
    /// Creates an aggregate reference from already-owned type and identifier values.
    pub fn new(aggregate_type: AggregateTypeOwned, aggregate_id: AggregateIdValue) -> Self {
        Self {
            aggregate_type,
            aggregate_id,
        }
    }

    /// Builds an aggregate reference from an aggregate ID and its static aggregate type.
    pub fn from_id<A: Aggregate>(aggregate_id: A::Id) -> Self {
        Self::new(
            AggregateTypeOwned::from(A::TYPE),
            AggregateIdValue::from(aggregate_id.value()),
        )
    }

    /// Builds an aggregate reference from an aggregate instance.
    ///
    /// Returns `AggregateRefError::MissingAggregateId` when the aggregate has
    /// not been initialized with state yet.
    pub fn try_from_aggregate<A: Aggregate>(value: &A) -> Result<Self, AggregateRefError> {
        let Some(id) = value.aggregate_id() else {
            return Err(AggregateRefError::MissingAggregateId);
        };

        Ok(Self::from_id::<A>(id))
    }
}

#[cfg(test)]
mod tests {
    use appletheia_domain::aggregate::{
        AggregateCore, AggregateError, AggregateState, AggregateStateError, UniqueConstraints,
    };
    use appletheia_domain::event::{EventName, EventPayload};
    use appletheia_domain::{Aggregate, AggregateApply, AggregateId, AggregateType};
    use serde::{Deserialize, Serialize};
    use std::fmt::{self, Display};
    use thiserror::Error;
    use uuid::Uuid;

    use super::AggregateRef;
    use crate::authorization::AggregateRefError;
    use crate::event::{AggregateIdValue, AggregateTypeOwned};

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

    impl Display for CounterId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug, Error)]
    enum CounterStateError {
        #[error(transparent)]
        AggregateState(#[from] AggregateStateError),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct CounterState {
        id: CounterId,
    }

    impl UniqueConstraints<CounterStateError> for CounterState {}

    impl AggregateState for CounterState {
        type Id = CounterId;
        type Error = CounterStateError;

        fn id(&self) -> Self::Id {
            self.id
        }
    }

    #[derive(Debug, Error)]
    enum CounterEventPayloadError {
        #[error(transparent)]
        Serde(#[from] serde_json::Error),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    enum CounterEventPayload {
        Opened,
    }

    impl EventPayload for CounterEventPayload {
        type Error = CounterEventPayloadError;

        fn name(&self) -> EventName {
            match self {
                Self::Opened => EventName::new("opened"),
            }
        }
    }

    #[derive(Debug, Error)]
    enum CounterError {
        #[error("aggregate error: {0}")]
        Aggregate(#[from] AggregateError<CounterId>),
    }

    #[derive(Clone, Debug, Default)]
    struct CounterAggregate {
        core: AggregateCore<CounterState, CounterEventPayload>,
    }

    impl AggregateApply<CounterEventPayload, CounterError> for CounterAggregate {
        fn apply(&mut self, _payload: &CounterEventPayload) -> Result<(), CounterError> {
            Ok(())
        }
    }

    impl Aggregate for CounterAggregate {
        type Id = CounterId;
        type State = CounterState;
        type EventPayload = CounterEventPayload;
        type Error = CounterError;

        const TYPE: AggregateType = AggregateType::new("counter");

        fn core(&self) -> &AggregateCore<Self::State, Self::EventPayload> {
            &self.core
        }

        fn core_mut(&mut self) -> &mut AggregateCore<Self::State, Self::EventPayload> {
            &mut self.core
        }
    }

    #[test]
    fn try_from_aggregate_returns_type_and_id() {
        let aggregate_id =
            CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let mut aggregate = CounterAggregate::default();
        aggregate.set_state(Some(CounterState { id: aggregate_id }));

        let aggregate_ref =
            AggregateRef::try_from_aggregate(&aggregate).expect("initialized aggregate");

        assert_eq!(aggregate_ref.aggregate_type.value(), "counter");
        assert_eq!(aggregate_ref.aggregate_id.value(), aggregate_id.value());
    }

    #[test]
    fn new_stores_type_and_id() {
        let aggregate_type = AggregateTypeOwned::from(CounterAggregate::TYPE);
        let aggregate_id = AggregateIdValue::from(Uuid::now_v7());

        let aggregate_ref = AggregateRef::new(aggregate_type.clone(), aggregate_id);

        assert_eq!(aggregate_ref.aggregate_type, aggregate_type);
        assert_eq!(aggregate_ref.aggregate_id, aggregate_id);
    }

    #[test]
    fn from_id_uses_static_aggregate_type() {
        let aggregate_id =
            CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");

        let aggregate_ref = AggregateRef::from_id::<CounterAggregate>(aggregate_id);

        assert_eq!(aggregate_ref.aggregate_type.value(), "counter");
        assert_eq!(aggregate_ref.aggregate_id.value(), aggregate_id.value());
    }

    #[test]
    fn try_from_aggregate_returns_error_when_id_is_missing() {
        let aggregate = CounterAggregate::default();

        let error = AggregateRef::try_from_aggregate(&aggregate)
            .expect_err("aggregate without state should be rejected");

        assert!(matches!(error, AggregateRefError::MissingAggregateId));
    }
}
