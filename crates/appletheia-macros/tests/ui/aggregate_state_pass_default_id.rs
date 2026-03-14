#![allow(dead_code, unused_imports)]

use std::convert::Infallible;

use appletheia_domain::{
    AggregateId, AggregateState, AggregateStateError, UniqueConstraints, UniqueValuesError,
};
use appletheia_macros::{aggregate_id, aggregate_state};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
enum CounterStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    UniqueValues(#[from] UniqueValuesError),
}

#[aggregate_id(error = Infallible)]
struct CounterId(Uuid);

#[aggregate_state(error = CounterStateError)]
struct CounterState {
    id: CounterId,
    counter: i32,
}

impl UniqueConstraints<CounterStateError> for CounterState {}

fn assert_aggregate_state<T: AggregateState<Id = CounterId>>() {}

fn main() {
    assert_aggregate_state::<CounterState>();
    let state = CounterState {
        id: CounterId::try_from_uuid(Uuid::nil()).unwrap(),
        counter: 1,
    };
    let _ = state.id();
}
