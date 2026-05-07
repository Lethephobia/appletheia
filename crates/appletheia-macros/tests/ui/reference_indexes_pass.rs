#![allow(dead_code, unused_imports)]

use std::convert::Infallible;

use appletheia_domain::{
    AggregateId, AggregateState, AggregateStateError, ReferenceIndexes, ReferenceKey,
    ReferenceValues, ReferenceValuesError, UniqueConstraints,
};
use appletheia_macros::{aggregate_id, aggregate_state, reference_indexes};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
enum CounterStateError {
    #[error(transparent)]
    AggregateState(#[from] AggregateStateError),

    #[error(transparent)]
    ReferenceValues(#[from] ReferenceValuesError),
}

#[aggregate_id(error = Infallible)]
struct CounterId(Uuid);

#[aggregate_id(error = Infallible)]
struct OwnerId(Uuid);

#[aggregate_state(error = CounterStateError)]
#[reference_indexes(entry(key = "owner_user", values = owner_user_values))]
struct CounterState {
    id: CounterId,
    owner_id: Option<OwnerId>,
}

impl UniqueConstraints<CounterStateError> for CounterState {}

fn owner_user_values(state: &CounterState) -> Result<Option<ReferenceValues>, CounterStateError> {
    Ok(state
        .owner_id
        .map(|owner_id| ReferenceValues::new(vec![owner_id]))
        .transpose()?)
}

fn assert_aggregate_state<T: AggregateState<Id = CounterId, Error = CounterStateError>>() {}

fn main() {
    assert_aggregate_state::<CounterState>();

    let owner_id = OwnerId::try_from_uuid(Uuid::now_v7()).unwrap();
    let state = CounterState {
        id: CounterId::try_from_uuid(Uuid::now_v7()).unwrap(),
        owner_id: Some(owner_id),
    };

    let reference_entries = state.reference_entries().unwrap();

    assert_eq!(CounterState::OWNER_USER_REF, ReferenceKey::new("owner_user"));
    assert_eq!(
        reference_entries
            .get(CounterState::OWNER_USER_REF)
            .map(ReferenceValues::len),
        Some(1)
    );
}
