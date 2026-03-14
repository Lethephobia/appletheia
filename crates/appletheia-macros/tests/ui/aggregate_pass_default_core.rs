#![allow(dead_code, unused_imports)]

use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use appletheia_domain::{
    Aggregate, AggregateApply, AggregateCore, AggregateError, AggregateId, AggregateState, EventName,
    AggregateStateError, EventPayload, UniqueConstraints, UniqueValuesError,
};
use appletheia_macros::aggregate;

#[derive(Debug, Error)]
#[error("counter id error")]
struct CounterIdError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
struct CounterId(Uuid);

impl Display for CounterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AggregateId for CounterId {
    type Error = CounterIdError;

    fn value(&self) -> Uuid {
        self.0
    }

    fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
struct CounterState {
    id: CounterId,
}

#[derive(Debug, Error)]
enum CounterStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    UniqueValues(#[from] UniqueValuesError),
}

impl UniqueConstraints<CounterStateError> for CounterState {}

impl AggregateState for CounterState {
    type Id = CounterId;
    type Error = CounterStateError;
    
    fn id(&self) -> Self::Id {
        self.id
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
enum CounterEventPayload {
    Created { id: CounterId },
}

impl CounterEventPayload {
    const CREATED: EventName = EventName::new("created");
}

impl EventPayload for CounterEventPayload {
    type Error = serde_json::Error;

    fn name(&self) -> EventName {
        match self {
            Self::Created { .. } => Self::CREATED,
        }
    }
}

type CounterError = AggregateError<CounterId>;

#[aggregate(type = "counter", error = CounterError)]
struct Counter {
    core: AggregateCore<CounterState, CounterEventPayload>,
}

impl AggregateApply<CounterEventPayload, CounterError> for Counter {
    fn apply(&mut self, payload: &CounterEventPayload) -> Result<(), CounterError> {
        match payload {
            CounterEventPayload::Created { id } => {
                self.set_state(Some(CounterState { id: *id }));
            }
        }
        Ok(())
    }
}

fn main() {}
