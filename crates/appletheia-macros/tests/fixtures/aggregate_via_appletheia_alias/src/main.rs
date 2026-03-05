use std::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use appletheia_macros::aggregate;

use theia::domain::{
    Aggregate, AggregateApply, AggregateCore, AggregateError, AggregateId, AggregateState,
    AggregateType, EventName, EventPayload,
};

#[derive(Debug, Error)]
enum CounterIdError {
    #[error("invalid")]
    Invalid,
}

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

    fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }

    fn value(&self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Error)]
enum CounterStateError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
struct CounterState {
    id: CounterId,
    counter: i32,
}

impl CounterState {
    fn new(id: CounterId, counter: i32) -> Self {
        Self { id, counter }
    }
}

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
    Json(#[from] serde_json::Error),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
enum CounterEventPayload {
    Created { id: CounterId },
    Increment(i32),
}

impl CounterEventPayload {
    const CREATED: EventName = EventName::new("created");
    const INCREMENT: EventName = EventName::new("increment");
}

impl EventPayload for CounterEventPayload {
    type Error = CounterEventPayloadError;

    fn name(&self) -> EventName {
        match self {
            Self::Created { .. } => Self::CREATED,
            Self::Increment(_) => Self::INCREMENT,
        }
    }
}

#[derive(Debug, Error)]
enum CounterError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CounterId>),

    #[error("invalid event payload")]
    InvalidEventPayload,
}

#[aggregate(type = "counter", error = CounterError)]
struct Counter {
    core: AggregateCore<CounterState, CounterEventPayload>,
}

impl AggregateApply<CounterEventPayload, CounterError> for Counter {
    fn apply(&mut self, payload: &CounterEventPayload) -> Result<(), CounterError> {
        match payload {
            CounterEventPayload::Created { id } => {
                if self.state().is_some() {
                    return Err(CounterError::InvalidEventPayload);
                }
                self.set_state(Some(CounterState::new(*id, 0)));
            }
            CounterEventPayload::Increment(delta) => {
                let state = self.state_required_mut()?;
                state.counter += delta;
            }
        }
        Ok(())
    }
}

fn main() {
    let _ = Counter::default();
}
