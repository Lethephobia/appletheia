use std::{fmt, fmt::Display};

use super::EventOutboxAttemptCountError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EventOutboxAttemptCount(i64);

impl EventOutboxAttemptCount {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn value(&self) -> i64 {
        self.0
    }

    pub fn increment(self) -> Self {
        Self(self.value() + 1)
    }

    pub fn checked_increment(self) -> Option<Self> {
        self.value().checked_add(1).map(Self)
    }

    pub fn try_increment(self) -> Result<Self, EventOutboxAttemptCountError> {
        self.checked_increment()
            .ok_or(EventOutboxAttemptCountError::Overflow)
    }
}

impl Default for EventOutboxAttemptCount {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<i64> for EventOutboxAttemptCount {
    type Error = EventOutboxAttemptCountError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(EventOutboxAttemptCountError::NegativeValue(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<EventOutboxAttemptCount> for i64 {
    fn from(value: EventOutboxAttemptCount) -> Self {
        value.value()
    }
}

impl Display for EventOutboxAttemptCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_starts_at_zero() {
        let count = EventOutboxAttemptCount::new();
        assert_eq!(count.value(), 0);
    }

    #[test]
    fn try_from_accepts_non_negative_values() {
        let count = EventOutboxAttemptCount::try_from(5).expect("expected valid count");
        assert_eq!(count.value(), 5);
    }

    #[test]
    fn try_from_rejects_negative_values() {
        let err = EventOutboxAttemptCount::try_from(-1).expect_err("expected negative value error");
        match err {
            EventOutboxAttemptCountError::NegativeValue(v) => assert_eq!(v, -1),
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn increment_increments_count() {
        let current = EventOutboxAttemptCount::new();
        let next = current.increment();
        assert_eq!(next.value(), 1);
    }

    #[test]
    fn checked_increment_handles_overflow() {
        let near_max = EventOutboxAttemptCount::try_from(i64::MAX - 1).unwrap();
        let next = near_max
            .checked_increment()
            .expect("should provide next value");
        assert_eq!(next.value(), i64::MAX);

        let max = EventOutboxAttemptCount::try_from(i64::MAX).unwrap();
        assert!(max.checked_increment().is_none());
    }

    #[test]
    fn try_increment_returns_error_on_overflow() {
        let near_max = EventOutboxAttemptCount::try_from(i64::MAX - 1).unwrap();
        let next = near_max.try_increment().expect("should provide next value");
        assert_eq!(next.value(), i64::MAX);

        let max = EventOutboxAttemptCount::try_from(i64::MAX).unwrap();
        let err = max.try_increment().expect_err("expected overflow error");
        match err {
            EventOutboxAttemptCountError::Overflow => {}
            _ => panic!("unexpected error variant"),
        }
    }
}
