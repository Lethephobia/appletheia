use std::{fmt, fmt::Display};

use super::EventOutboxRelayInstanceError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EventOutboxRelayInstanceId(String);

impl EventOutboxRelayInstanceId {
    pub fn new(value: String) -> Result<Self, EventOutboxRelayInstanceError> {
        if value.is_empty() {
            Err(EventOutboxRelayInstanceError::EmptyInstanceId)
        } else {
            Ok(Self(value))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Display for EventOutboxRelayInstanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_value() {
        let err = EventOutboxRelayInstanceId::new(String::new()).expect_err("expected error");
        matches!(err, EventOutboxRelayInstanceError::EmptyInstanceId);
    }

    #[test]
    fn new_accepts_non_empty_value() {
        let id = EventOutboxRelayInstanceId::new("instance-1".to_string()).unwrap();
        assert_eq!(id.value(), "instance-1");
    }
}
