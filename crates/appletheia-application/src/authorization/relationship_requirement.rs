use appletheia_domain::Aggregate;

use super::{AggregateRef, RelationRef, RelationRefOwned};

/// Describes relationship checks required for authorization.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum RelationshipRequirement {
    /// Requires the principal to satisfy a relation on a specific aggregate.
    Check {
        /// The aggregate on which the relation is evaluated.
        aggregate: AggregateRef,
        /// The relation the principal must satisfy on the aggregate.
        relation: RelationRefOwned,
    },
    /// Requires all contained relationship requirements to be satisfied.
    All(Vec<RelationshipRequirement>),
    /// Requires at least one contained relationship requirement to be satisfied.
    Any(Vec<RelationshipRequirement>),
    /// Requires the contained relationship requirement to be unsatisfied.
    Not(Box<RelationshipRequirement>),
}

impl RelationshipRequirement {
    /// Creates a relationship check requirement for the given aggregate id and relation.
    pub fn check<A>(aggregate_id: A::Id, relation: RelationRef) -> Self
    where
        A: Aggregate,
    {
        Self::Check {
            aggregate: AggregateRef::from_id::<A>(aggregate_id),
            relation: RelationRefOwned::from(relation),
        }
    }
}

#[cfg(test)]
mod tests {
    use appletheia_domain::AggregateType;
    use appletheia_domain::aggregate::{
        AggregateCore, AggregateError, AggregateId, AggregateState, AggregateStateError,
        UniqueConstraints,
    };
    use appletheia_domain::event::{EventName, EventPayload};
    use appletheia_domain::{Aggregate, AggregateApply};
    use serde::{Deserialize, Serialize};
    use std::fmt::{self, Display};
    use thiserror::Error;

    use super::RelationshipRequirement;
    use crate::authorization::{AggregateRef, RelationName, RelationRef, RelationRefOwned};
    use crate::event::{AggregateIdValue, AggregateTypeOwned};

    const TEST_AGGREGATE_TYPE: AggregateType = AggregateType::new("test_aggregate");
    const TEST_RELATION: RelationRef =
        RelationRef::new(TEST_AGGREGATE_TYPE, RelationName::new("owner"));

    #[test]
    fn check_builds_owned_relation_from_static_relation() {
        let aggregate_id = TestId(uuid::Uuid::now_v7());
        let requirement =
            RelationshipRequirement::check::<TestAggregate>(aggregate_id, TEST_RELATION);

        assert_eq!(
            requirement,
            RelationshipRequirement::Check {
                aggregate: AggregateRef::new(
                    AggregateTypeOwned::from(TEST_AGGREGATE_TYPE),
                    AggregateIdValue::from(aggregate_id.value()),
                ),
                relation: RelationRefOwned::from(TEST_RELATION),
            }
        );
    }

    #[derive(Debug, Error, Eq, PartialEq)]
    enum TestIdError {
        #[error("nil uuid is not allowed")]
        NilUuid,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    struct TestId(uuid::Uuid);

    impl AggregateId for TestId {
        type Error = TestIdError;

        fn value(&self) -> uuid::Uuid {
            self.0
        }

        fn try_from_uuid(value: uuid::Uuid) -> Result<Self, Self::Error> {
            if value.is_nil() {
                return Err(TestIdError::NilUuid);
            }

            Ok(Self(value))
        }
    }

    impl Display for TestId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug, Error)]
    enum TestStateError {
        #[error(transparent)]
        AggregateState(#[from] AggregateStateError),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct TestState {
        id: TestId,
    }

    impl UniqueConstraints<TestStateError> for TestState {}

    impl AggregateState for TestState {
        type Id = TestId;
        type Error = TestStateError;

        fn id(&self) -> Self::Id {
            self.id
        }
    }

    #[derive(Debug, Error)]
    enum TestPayloadError {
        #[error(transparent)]
        Serde(#[from] serde_json::Error),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    enum TestPayload {
        Opened,
    }

    impl EventPayload for TestPayload {
        type Error = TestPayloadError;

        fn name(&self) -> EventName {
            EventName::new("opened")
        }
    }

    #[derive(Debug, Error)]
    enum TestAggregateError {
        #[error(transparent)]
        Aggregate(#[from] AggregateError<TestId>),
    }

    #[derive(Clone, Debug, Default)]
    struct TestAggregate {
        core: AggregateCore<TestState, TestPayload>,
    }

    impl AggregateApply<TestPayload, TestAggregateError> for TestAggregate {
        fn apply(&mut self, _payload: &TestPayload) -> Result<(), TestAggregateError> {
            Ok(())
        }
    }

    impl Aggregate for TestAggregate {
        type Id = TestId;
        type State = TestState;
        type EventPayload = TestPayload;
        type Error = TestAggregateError;

        const TYPE: AggregateType = TEST_AGGREGATE_TYPE;

        fn core(&self) -> &AggregateCore<Self::State, Self::EventPayload> {
            &self.core
        }

        fn core_mut(&mut self) -> &mut AggregateCore<Self::State, Self::EventPayload> {
            &mut self.core
        }
    }
}
