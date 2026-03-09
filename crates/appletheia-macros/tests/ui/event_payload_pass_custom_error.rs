#![allow(dead_code, unused_imports)]

use std::error::Error;
use std::fmt::{self, Display};

use appletheia_domain::{EventName, EventPayload};
use appletheia_macros::event_payload;

#[derive(Debug)]
struct CounterEventPayloadError;

impl Display for CounterEventPayloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "counter event payload error")
    }
}

impl Error for CounterEventPayloadError {}

impl From<serde_json::Error> for CounterEventPayloadError {
    fn from(_: serde_json::Error) -> Self {
        Self
    }
}

#[event_payload(error = CounterEventPayloadError)]
enum CounterEventPayload {
    Opened,
    UserRenamed { new_name: String },
}

fn assert_event_payload<T: EventPayload<Error = CounterEventPayloadError>>() {}

fn main() {
    assert_event_payload::<CounterEventPayload>();
    let payload = CounterEventPayload::UserRenamed {
        new_name: "alice".to_owned(),
    };
    let _ = payload.name();
    let _: EventName = CounterEventPayload::OPENED;
    let _: EventName = CounterEventPayload::USER_RENAMED;
}
