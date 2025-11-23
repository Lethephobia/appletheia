use std::{fmt, fmt::Display};

use super::OutboxRelayInstanceError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct OutboxRelayInstanceId(String);

impl OutboxRelayInstanceId {
    pub fn new(value: String) -> Result<Self, OutboxRelayInstanceError> {
        if value.is_empty() {
            Err(OutboxRelayInstanceError::EmptyInstanceId)
        } else {
            Ok(Self(value))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Display for OutboxRelayInstanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_value() {
        let err = OutboxRelayInstanceId::new(String::new()).expect_err("expected error");
        matches!(err, OutboxRelayInstanceError::EmptyInstanceId);
    }

    #[test]
    fn new_accepts_non_empty_value() {
        let id = OutboxRelayInstanceId::new("instance-1".to_string()).unwrap();
        assert_eq!(id.value(), "instance-1");
    }
}
