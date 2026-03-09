#![allow(dead_code, unused_imports)]

use appletheia_domain::{EventName, EventPayload};
use appletheia_macros::event_payload;

#[event_payload(error = serde_json::Error)]
enum CounterEventPayload {
    Created { id: u64 },
    Increment(i32),
    Reset,
}

fn assert_event_payload<T: EventPayload<Error = serde_json::Error>>() {}

fn main() {
    assert_event_payload::<CounterEventPayload>();
    let payload = CounterEventPayload::Created { id: 1 };
    let _ = payload.name();
    let _: EventName = CounterEventPayload::CREATED;
    let _: EventName = CounterEventPayload::INCREMENT;
    let _: EventName = CounterEventPayload::RESET;
}
