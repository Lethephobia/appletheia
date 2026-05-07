#![allow(dead_code, unused_imports)]

use std::convert::Infallible;

use appletheia_domain::{
    AggregateId, AggregateState, AggregateStateError, ReferenceIndexes, UniqueConstraints,
    UniqueKey, UniqueValue, UniqueValues, UniqueValuesError,
};
use appletheia_macros::{aggregate_id, aggregate_state, unique_constraints};
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
#[unique_constraints(
    entry(key = "email", values = email_values),
    entry(key = "username", value = username_value),
    entry(key = "phone_number", values = phone_number_values),
)]
struct CounterState {
    id: CounterId,
    email: Option<String>,
    username: Option<String>,
    phone_number: Option<String>,
}

impl ReferenceIndexes<CounterStateError> for CounterState {}

fn email_values(state: &CounterState) -> Result<Option<UniqueValues>, CounterStateError> {
    match state.email.as_deref() {
        Some(email) => Ok(Some(single_value(email))),
        None => Ok(None),
    }
}

fn phone_number_values(state: &CounterState) -> Result<Option<UniqueValues>, CounterStateError> {
    match state.phone_number.as_deref() {
        Some(phone_number) => Ok(Some(single_value(phone_number))),
        None => Ok(None),
    }
}

fn username_value(state: &CounterState) -> Result<Option<UniqueValue>, CounterStateError> {
    Ok(state
        .username
        .as_deref()
        .map(|username| UniqueValue::from_strings([username]).expect("valid unique value")))
}

fn single_value(input: &str) -> UniqueValues {
    let value = UniqueValue::from_strings([input]).expect("valid value");
    UniqueValues::new(vec![value]).expect("unique values should be valid")
}

fn assert_aggregate_state<T: AggregateState<Id = CounterId, Error = CounterStateError>>() {}

fn main() {
    assert_aggregate_state::<CounterState>();

    let state = CounterState {
        id: CounterId::try_from_uuid(Uuid::nil()).unwrap(),
        email: Some("foo@example.com".to_owned()),
        username: Some("foo".to_owned()),
        phone_number: None,
    };

    let unique_entries = state.unique_entries().unwrap();

    assert_eq!(CounterState::EMAIL_KEY, UniqueKey::new("email"));
    assert_eq!(CounterState::USERNAME_KEY, UniqueKey::new("username"));
    assert_eq!(CounterState::PHONE_NUMBER_KEY, UniqueKey::new("phone_number"));
    assert_eq!(
        unique_entries.get(CounterState::EMAIL_KEY).map(UniqueValues::len),
        Some(1)
    );
    assert_eq!(
        unique_entries
            .get(CounterState::USERNAME_KEY)
            .map(UniqueValues::len),
        Some(1)
    );
    assert_eq!(unique_entries.get(CounterState::PHONE_NUMBER_KEY), None);
}
