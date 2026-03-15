#![allow(dead_code, unused_imports)]

use appletheia_domain::AggregateId;
use appletheia_macros::aggregate_id;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
#[error("counter id error")]
struct CounterIdError;

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
