#![allow(dead_code, unused_imports)]

use std::convert::Infallible;

use appletheia_domain::{
    AggregateId, AggregateState, AggregateStateError, UniqueConstraints, UniqueKey, UniqueValue,
    UniqueValuePart, UniqueValues, UniqueValuesError,
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
    entry(key = "phone_number", values = phone_number_values),
)]
struct CounterState {
    id: CounterId,
    email: Option<String>,
    phone_number: Option<String>,
}

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

fn single_value(input: &str) -> UniqueValues {
    let value = UniqueValue::new(vec![UniqueValuePart::try_from(input).expect("valid part")])
        .expect("valid value");
    UniqueValues::new(vec![value]).expect("unique values should be valid")
}

fn assert_aggregate_state<T: AggregateState<Id = CounterId, Error = CounterStateError>>() {}

fn main() {
    assert_aggregate_state::<CounterState>();

    let state = CounterState {
        id: CounterId::try_from_uuid(Uuid::nil()).unwrap(),
        email: Some("foo@example.com".to_owned()),
        phone_number: None,
    };

    let unique_entries = state.unique_entries().unwrap();

    assert_eq!(
        unique_entries
            .get(UniqueKey::new("email"))
            .map(UniqueValues::len),
        Some(1)
    );
    assert_eq!(unique_entries.get(UniqueKey::new("phone_number")), None);
}
