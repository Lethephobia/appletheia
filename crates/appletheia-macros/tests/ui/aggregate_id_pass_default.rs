#![allow(dead_code, unused_imports)]

use appletheia_domain::AggregateId;
use appletheia_macros::aggregate_id;
use uuid::Uuid;

#[aggregate_id]
struct CounterId(Uuid);

fn assert_aggregate_id<T: AggregateId>() {}

fn main() {
    assert_aggregate_id::<CounterId>();
    let _ = CounterId::try_from_uuid(Uuid::nil()).unwrap();
}

