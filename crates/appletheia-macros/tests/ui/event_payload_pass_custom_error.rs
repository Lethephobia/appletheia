#![allow(dead_code, unused_imports)]

use appletheia_domain::{EventName, EventPayload};
use appletheia_macros::event_payload;
use thiserror::Error;

#[derive(Debug, Error)]
enum CounterEventPayloadError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
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
