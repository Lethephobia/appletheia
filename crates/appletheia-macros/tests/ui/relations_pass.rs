use appletheia_application::authorization::{
    AuthorizationTypeDefinition, Relation, RelationName, Relations, UsersetExpr,
};
use appletheia_domain::aggregate::{
    AggregateCore, AggregateError, AggregateId, AggregateState, AggregateStateError,
    UniqueConstraints,
};
use appletheia_domain::event::{EventName, EventPayload};
use appletheia_domain::{Aggregate, AggregateApply, AggregateType};
use appletheia_macros::relations;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

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

#[derive(Debug)]
struct CounterError;

impl std::fmt::Display for CounterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("counter error")
    }
}

impl std::error::Error for CounterError {}

impl From<AggregateError<CounterId>> for CounterError {
    fn from(_value: AggregateError<CounterId>) -> Self {
        Self
    }
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

struct ViewerRelation;

impl Relation for ViewerRelation {
    const NAME: RelationName = RelationName::new("viewer");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}

struct EditorRelation;

impl Relation for EditorRelation {
    const NAME: RelationName = RelationName::new("editor");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::ComputedUserset {
            relation: ViewerRelation::NAME.into(),
        }
    }
}

#[relations(aggregate = CounterAggregate, relations = [ViewerRelation, EditorRelation])]
struct CounterRelations;

fn main() {
    let definition = CounterRelations.build();
    let _ = CounterRelations::AGGREGATE_TYPE;
    let _ = AuthorizationTypeDefinition::default();
    let _ = definition;
}
