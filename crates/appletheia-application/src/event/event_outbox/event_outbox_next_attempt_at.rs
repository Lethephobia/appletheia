use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

use super::EventOutboxRetryDelay;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EventOutboxNextAttemptAt(DateTime<Utc>);

impl EventOutboxNextAttemptAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn next(self, backoff: EventOutboxRetryDelay) -> Self {
        Self(self.value() + backoff.value())
    }
}

impl Default for EventOutboxNextAttemptAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for EventOutboxNextAttemptAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<EventOutboxNextAttemptAt> for DateTime<Utc> {
    fn from(value: EventOutboxNextAttemptAt) -> Self {
        value.0
    }
}

impl Display for EventOutboxNextAttemptAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn default_uses_now() {
        let before = Utc::now();
        let next_attempt = EventOutboxNextAttemptAt::default();
        let after = Utc::now();

        let value = next_attempt.value();
        assert!(value >= before, "expected {value} to be after {before}");
        assert!(value <= after, "expected {value} to be before {after}");
    }

    #[test]
    fn value_returns_inner_datetime() {
        let t = Utc::now();
        let next_attempt = EventOutboxNextAttemptAt::from(t);
        assert_eq!(next_attempt.value(), t);
    }

    #[test]
    fn next_applies_backoff_duration() {
        let base = Utc::now();
        let next_attempt = EventOutboxNextAttemptAt::from(base);
        let backoff = EventOutboxRetryDelay::from(Duration::seconds(30));

        let advanced = next_attempt.next(backoff);
        assert_eq!(advanced.value(), base + backoff.value());
    }

    #[test]
    fn conversions_round_trip() {
        let t = Utc::now();
        let wrapped: EventOutboxNextAttemptAt = t.into();
        let back: DateTime<Utc> = wrapped.into();
        assert_eq!(back, t);
    }

    #[test]
    fn display_matches_inner_datetime() {
        let t = Utc::now();
        let wrapped = EventOutboxNextAttemptAt::from(t);
        assert_eq!(wrapped.to_string(), t.to_string());
    }
}
