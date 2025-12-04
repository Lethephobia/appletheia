use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

use super::OutboxRetryDelay;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct OutboxNextAttemptAt(DateTime<Utc>);

impl OutboxNextAttemptAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn next(self, backoff: OutboxRetryDelay) -> Self {
        Self(self.value() + backoff.value())
    }
}

impl Default for OutboxNextAttemptAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for OutboxNextAttemptAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<OutboxNextAttemptAt> for DateTime<Utc> {
    fn from(value: OutboxNextAttemptAt) -> Self {
        value.0
    }
}

impl Display for OutboxNextAttemptAt {
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
        let next_attempt = OutboxNextAttemptAt::default();
        let after = Utc::now();

        let value = next_attempt.value();
        assert!(value >= before, "expected {value} to be after {before}");
        assert!(value <= after, "expected {value} to be before {after}");
    }

    #[test]
    fn value_returns_inner_datetime() {
        let t = Utc::now();
        let next_attempt = OutboxNextAttemptAt::from(t);
        assert_eq!(next_attempt.value(), t);
    }

    #[test]
    fn next_applies_backoff_duration() {
        let base = Utc::now();
        let next_attempt = OutboxNextAttemptAt::from(base);
        let backoff = OutboxRetryDelay::from(Duration::seconds(30));

        let advanced = next_attempt.next(backoff);
        assert_eq!(advanced.value(), base + backoff.value());
    }

    #[test]
    fn conversions_round_trip() {
        let t = Utc::now();
        let wrapped: OutboxNextAttemptAt = t.into();
        let back: DateTime<Utc> = wrapped.into();
        assert_eq!(back, t);
    }

    #[test]
    fn display_matches_inner_datetime() {
        let t = Utc::now();
        let wrapped = OutboxNextAttemptAt::from(t);
        assert_eq!(wrapped.to_string(), t.to_string());
    }
}
