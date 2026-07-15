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
#[reference_indexes(
    entry(key = "owner_user", values = owner_user_values),
    entry(key = "approver_user", value = approver_user_value),
)]
struct CounterState {
    id: CounterId,
    owner_id: Option<OwnerId>,
    approver_id: Option<OwnerId>,
}

impl UniqueConstraints<CounterStateError> for CounterState {}

fn owner_user_values(state: &CounterState, _aggregate_id: uuid::Uuid) -> Result<Option<ReferenceValues>, CounterStateError> {
    Ok(state
        .owner_id
        .map(|owner_id| ReferenceValues::new(vec![owner_id]))
        .transpose()?)
}

fn approver_user_value(state: &CounterState, _aggregate_id: uuid::Uuid) -> Result<Option<OwnerId>, CounterStateError> {
    Ok(state.approver_id)
}

fn assert_aggregate_state<T: AggregateState<Error = CounterStateError>>() {}

fn main() {
    assert_aggregate_state::<CounterState>();

    let owner_id = OwnerId::try_from_uuid(Uuid::now_v7()).unwrap();
    let approver_id = OwnerId::try_from_uuid(Uuid::now_v7()).unwrap();
    let state = CounterState {
        id: CounterId::try_from_uuid(Uuid::now_v7()).unwrap(),
        owner_id: Some(owner_id),
        approver_id: Some(approver_id),
    };

    let reference_entries = state.reference_entries(uuid::Uuid::now_v7()).unwrap();

    assert_eq!(CounterState::OWNER_USER_REF, ReferenceKey::new("owner_user"));
    assert_eq!(
        CounterState::APPROVER_USER_REF,
        ReferenceKey::new("approver_user")
    );
    assert_eq!(
        reference_entries
            .get(CounterState::OWNER_USER_REF)
            .map(ReferenceValues::len),
        Some(1)
    );
    assert_eq!(
        reference_entries
            .get(CounterState::APPROVER_USER_REF)
            .map(ReferenceValues::len),
        Some(1)
    );
}
