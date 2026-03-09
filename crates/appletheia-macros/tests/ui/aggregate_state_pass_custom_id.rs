#![allow(dead_code, unused_imports)]

use std::convert::Infallible;

use appletheia_domain::{AggregateId, AggregateState};
use appletheia_macros::{aggregate_id, aggregate_state};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
enum CounterStateError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[aggregate_id(error = Infallible)]
struct CounterId(Uuid);

#[aggregate_state(id = aggregate_id, error = CounterStateError)]
struct CounterState {
    aggregate_id: CounterId,
    counter: i32,
}

fn assert_aggregate_state<T: AggregateState<Id = CounterId, Error = CounterStateError>>() {}

fn main() {
    assert_aggregate_state::<CounterState>();
    let state = CounterState {
        aggregate_id: CounterId::try_from_uuid(Uuid::nil()).unwrap(),
        counter: 1,
    };
    let _ = state.id();
}
