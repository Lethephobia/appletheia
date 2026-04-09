use appletheia_domain::Aggregate;
use serde::{Deserialize, Serialize};

use crate::event::AggregateTypeOwned;

use super::{AggregateRef, RelationRef, RelationRefOwned};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum RelationshipSubject {
    /// `<type>:<id>`
    Aggregate(AggregateRef),

    /// `<type>:*`
    Wildcard { aggregate_type: AggregateTypeOwned },

    /// `<type>:<id>#<relation>`
    AggregateSet {
        aggregate: AggregateRef,
        relation: RelationRefOwned,
    },
}

impl RelationshipSubject {
    pub fn aggregate<A: Aggregate>(aggregate_id: A::Id) -> Self {
        Self::Aggregate(AggregateRef::from_id::<A>(aggregate_id))
    }

    pub fn wildcard<A: Aggregate>() -> Self {
        Self::Wildcard {
            aggregate_type: AggregateTypeOwned::from(A::TYPE),
        }
    }

    pub fn aggregate_set<A: Aggregate>(aggregate_id: A::Id, relation: RelationRef) -> Self {
        Self::AggregateSet {
            aggregate: AggregateRef::from_id::<A>(aggregate_id),
            relation: RelationRefOwned::from(relation),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{self, Display};

    use appletheia_domain::aggregate::{
        AggregateCore, AggregateError, AggregateId, AggregateState, AggregateStateError,
        UniqueConstraints,
    };
    use appletheia_domain::event::{EventName, EventPayload};
    use appletheia_domain::{Aggregate, AggregateApply, AggregateType};
    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use uuid::Uuid;

    use super::RelationshipSubject;
    use crate::authorization::{AggregateRef, RelationName, RelationRef, RelationRefOwned};
    use crate::event::AggregateTypeOwned;

    const TEST_RELATION: RelationRef =
        RelationRef::new(TestAggregate::TYPE, RelationName::new("member"));

    #[test]
    fn aggregate_builds_aggregate_subject() {
        let aggregate_id = TestId::try_from_uuid(Uuid::now_v7()).expect("valid uuid");

        assert_eq!(
            RelationshipSubject::aggregate::<TestAggregate>(aggregate_id),
            RelationshipSubject::Aggregate(AggregateRef::from_id::<TestAggregate>(aggregate_id)),
        );
    }

    #[test]
    fn wildcard_builds_wildcard_subject() {
        assert_eq!(
            RelationshipSubject::wildcard::<TestAggregate>(),
            RelationshipSubject::Wildcard {
                aggregate_type: AggregateTypeOwned::from(TestAggregate::TYPE),
            },
        );
    }

    #[test]
    fn aggregate_set_builds_aggregate_set_subject() {
        let aggregate_id = TestId::try_from_uuid(Uuid::now_v7()).expect("valid uuid");

        assert_eq!(
            RelationshipSubject::aggregate_set::<TestAggregate>(aggregate_id, TEST_RELATION),
            RelationshipSubject::AggregateSet {
                aggregate: AggregateRef::from_id::<TestAggregate>(aggregate_id),
                relation: RelationRefOwned::from(TEST_RELATION),
            },
        );
    }

    #[derive(Debug, Error, Eq, PartialEq)]
    enum TestIdError {
        #[error("nil uuid is not allowed")]
        NilUuid,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    struct TestId(Uuid);

    impl AggregateId for TestId {
        type Error = TestIdError;

        fn value(&self) -> Uuid {
            self.0
        }

        fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
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
    enum TestEventPayloadError {
        #[error(transparent)]
        Serde(#[from] serde_json::Error),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    enum TestEventPayload {
        Opened,
    }

    impl EventPayload for TestEventPayload {
        type Error = TestEventPayloadError;

        fn name(&self) -> EventName {
            match self {
                Self::Opened => EventName::new("opened"),
            }
        }
    }

    #[derive(Debug, Error)]
    enum TestError {
        #[error("aggregate error: {0}")]
        Aggregate(#[from] AggregateError<TestId>),
    }

    #[derive(Clone, Debug, Default)]
    struct TestAggregate {
        core: AggregateCore<TestState, TestEventPayload>,
    }

    impl AggregateApply<TestEventPayload, TestError> for TestAggregate {
        fn apply(&mut self, _payload: &TestEventPayload) -> Result<(), TestError> {
            Ok(())
        }
    }

    impl Aggregate for TestAggregate {
        type Id = TestId;
        type State = TestState;
        type EventPayload = TestEventPayload;
        type Error = TestError;

        const TYPE: AggregateType = AggregateType::new("test_aggregate");

        fn core(&self) -> &AggregateCore<Self::State, Self::EventPayload> {
            &self.core
        }

        fn core_mut(&mut self) -> &mut AggregateCore<Self::State, Self::EventPayload> {
            &mut self.core
        }
    }
}
