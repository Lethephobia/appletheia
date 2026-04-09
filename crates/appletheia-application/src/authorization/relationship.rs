use appletheia_domain::Aggregate;
use serde::{Deserialize, Serialize};

use super::{AggregateRef, RelationRef, RelationRefOwned, RelationshipSubject};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Relationship {
    pub aggregate: AggregateRef,
    pub relation: RelationRefOwned,
    pub subject: RelationshipSubject,
}

impl Relationship {
    pub fn new<A: Aggregate>(
        aggregate_id: A::Id,
        relation: RelationRef,
        subject: RelationshipSubject,
    ) -> Self {
        Self {
            aggregate: AggregateRef::from_id::<A>(aggregate_id),
            relation: RelationRefOwned::from(relation),
            subject,
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

    use super::Relationship;
    use crate::authorization::{
        AggregateRef, RelationName, RelationRef, RelationRefOwned, RelationshipSubject,
    };

    const TEST_RELATION: RelationRef =
        RelationRef::new(TestAggregate::TYPE, RelationName::new("member"));

    #[test]
    fn new_builds_relationship_from_aggregate_id() {
        let aggregate_id = TestId::try_from_uuid(Uuid::now_v7()).expect("valid uuid");
        let subject_id = TestId::try_from_uuid(Uuid::now_v7()).expect("valid uuid");

        assert_eq!(
            Relationship::new::<TestAggregate>(
                aggregate_id,
                TEST_RELATION,
                RelationshipSubject::aggregate::<TestAggregate>(subject_id),
            ),
            Relationship {
                aggregate: AggregateRef::from_id::<TestAggregate>(aggregate_id),
                relation: RelationRefOwned::from(TEST_RELATION),
                subject: RelationshipSubject::aggregate::<TestAggregate>(subject_id),
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
