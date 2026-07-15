#![allow(dead_code, unused_imports)]

use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use appletheia_domain::{
    Aggregate, AggregateApply, AggregateCore, AggregateError, AggregateId, AggregateState,
    AggregateStateError, EventName, EventPayload, ReferenceIndexes, UniqueConstraints,
    UniqueValuesError,
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

    fn new() -> Self {
        Self(Uuid::now_v7())
    }

    fn value(&self) -> Uuid {
        self.0
    }

    fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
struct CounterState;

#[derive(Debug, Error)]
enum CounterStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    UniqueValues(#[from] UniqueValuesError),
}

impl UniqueConstraints<CounterStateError> for CounterState {}
impl ReferenceIndexes<CounterStateError> for CounterState {}

impl AggregateState for CounterState {
    type Error = CounterStateError;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
enum CounterEventPayload {
    Created,
}

impl CounterEventPayload {
    const CREATED: EventName = EventName::new("created");
}

impl EventPayload for CounterEventPayload {
    type Error = serde_json::Error;

    fn name(&self) -> EventName {
        match self {
            Self::Created => Self::CREATED,
        }
    }
}

#[derive(Debug, Error)]
enum CounterError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CounterId>),

    #[error(transparent)]
    State(#[from] CounterStateError),
}

#[aggregate(type = "counter", core = "inner", error = CounterError)]
struct Counter {
    inner: AggregateCore<CounterId, CounterState, CounterEventPayload>,
}

impl AggregateApply<CounterEventPayload, CounterError> for Counter {
    fn apply(&mut self, payload: &CounterEventPayload) -> Result<(), CounterError> {
        match payload {
            CounterEventPayload::Created => {
                self.set_state(Some(CounterState));
            }
        }
        Ok(())
    }
}

fn main() {}
