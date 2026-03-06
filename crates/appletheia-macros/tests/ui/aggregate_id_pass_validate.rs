#![allow(dead_code, unused_imports)]

use std::error::Error;
use std::fmt::{self, Display};

use appletheia_domain::AggregateId;
use appletheia_macros::aggregate_id;
use uuid::Uuid;

#[derive(Debug)]
struct CounterIdError;

impl Display for CounterIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "counter id error")
    }
}

impl Error for CounterIdError {}

fn validate_counter_id(_value: Uuid) -> Result<(), CounterIdError> {
    Ok(())
}

#[aggregate_id(error = CounterIdError, validate = validate_counter_id)]
struct CounterId(Uuid);

fn assert_aggregate_id<T: AggregateId>() {}

fn main() {
    assert_aggregate_id::<CounterId>();
    let _ = CounterId::try_from_uuid(Uuid::nil()).unwrap();
}

