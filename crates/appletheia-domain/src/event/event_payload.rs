use std::{error::Error, fmt::Debug};

use serde::Serialize;
use serde::de::DeserializeOwned;

use super::EventName;

/// Represents the domain payload carried by an event.
///
/// Implementations provide a stable event name and JSON conversion helpers used
/// at serialization boundaries.
pub trait EventPayload:
    Clone + Debug + Eq + Serialize + DeserializeOwned + Send + Sync + 'static
{
    type Error: Error + From<serde_json::Error> + Send + Sync + 'static;

    /// Returns the stable name of this event payload.
    fn name(&self) -> EventName;

    /// Deserializes the payload from a JSON value.
    fn try_from_json_value(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value).map_err(serde_json::Error::into)
    }

    /// Serializes the payload into a JSON value.
    fn into_json_value(self) -> Result<serde_json::Value, Self::Error> {
        serde_json::to_value(self).map_err(serde_json::Error::into)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use thiserror::Error;

    use super::EventPayload;
    use crate::event::EventName;

    #[derive(Debug, Error)]
    enum CounterEventPayloadError {
        #[error(transparent)]
        Serde(#[from] serde_json::Error),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "type", content = "data", rename_all = "snake_case")]
    enum CounterEventPayload {
        Opened,
        Incremented { amount: i32 },
    }

    impl CounterEventPayload {
        const OPENED: EventName = EventName::new("opened");
        const INCREMENTED: EventName = EventName::new("incremented");
    }

    impl EventPayload for CounterEventPayload {
        type Error = CounterEventPayloadError;

        fn name(&self) -> EventName {
            match self {
                Self::Opened => Self::OPENED,
                Self::Incremented { .. } => Self::INCREMENTED,
            }
        }
    }

    #[test]
    fn name_returns_stable_event_name() {
        let payload = CounterEventPayload::Incremented { amount: 2 };

        assert_eq!(payload.name(), EventName::new("incremented"));
        assert_eq!(CounterEventPayload::OPENED, EventName::new("opened"));
        assert_eq!(
            CounterEventPayload::INCREMENTED,
            EventName::new("incremented")
        );
    }

    #[test]
    fn try_from_json_value_deserializes_payload() {
        let value = serde_json::json!({
            "type": "incremented",
            "data": {
                "amount": 4
            }
        });

        let payload =
            CounterEventPayload::try_from_json_value(value).expect("json value should deserialize");

        assert_eq!(payload, CounterEventPayload::Incremented { amount: 4 });
    }

    #[test]
    fn try_from_json_value_propagates_serde_errors() {
        let value = serde_json::json!({
            "type": "incremented",
            "data": {
                "amount": "invalid"
            }
        });

        let error =
            CounterEventPayload::try_from_json_value(value).expect_err("invalid json should fail");

        assert!(matches!(error, CounterEventPayloadError::Serde(_)));
    }

    #[test]
    fn into_json_value_serializes_payload() {
        let payload = CounterEventPayload::Incremented { amount: 7 };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("incremented"));
        assert_eq!(value["data"]["amount"], serde_json::json!(7));
    }
}
