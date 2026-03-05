#![allow(dead_code, unused_imports)]

use std::error::Error;
use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use appletheia_domain::{
    Aggregate, AggregateApply, AggregateCore, AggregateError, AggregateId, AggregateState, EventName,
    EventPayload,
};
use appletheia_macros::aggregate;

#[derive(Debug)]
struct CounterIdError;

impl Display for CounterIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "counter id error")
    }
}

impl Error for CounterIdError {}

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

impl AggregateState for CounterState {
    type Id = CounterId;
    type Error = serde_json::Error;

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
