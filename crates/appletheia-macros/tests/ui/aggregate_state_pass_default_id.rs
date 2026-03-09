#![allow(dead_code, unused_imports)]

use appletheia_domain::{AggregateId, AggregateState};
use appletheia_macros::{aggregate_id, aggregate_state};
use uuid::Uuid;

#[aggregate_id]
struct CounterId(Uuid);

#[aggregate_state]
struct CounterState {
    id: CounterId,
    counter: i32,
}

fn assert_aggregate_state<T: AggregateState<Id = CounterId>>() {}

fn main() {
    assert_aggregate_state::<CounterState>();
    let state = CounterState {
        id: CounterId::try_from_uuid(Uuid::nil()).unwrap(),
        counter: 1,
    };
    let _ = state.id();
}
