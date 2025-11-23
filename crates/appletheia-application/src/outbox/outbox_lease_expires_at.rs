use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

use super::OutboxLeaseDuration;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct OutboxLeaseExpiresAt(DateTime<Utc>);

impl OutboxLeaseExpiresAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(self) -> DateTime<Utc> {
        self.0
    }

    pub fn from_now(expires_in: OutboxLeaseDuration) -> Self {
        Self(Utc::now() + expires_in.value())
    }
}

impl Default for OutboxLeaseExpiresAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for OutboxLeaseExpiresAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<OutboxLeaseExpiresAt> for DateTime<Utc> {
    fn from(value: OutboxLeaseExpiresAt) -> Self {
        value.0
    }
}

impl Display for OutboxLeaseExpiresAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, TimeZone};

    #[test]
    fn default_uses_now() {
        let before = Utc::now();
        let lease_expires_at = OutboxLeaseExpiresAt::default();
        let after = Utc::now();

        let value = lease_expires_at.value();
        assert!(value >= before, "expected {value} to be after {before}");
        assert!(value <= after, "expected {value} to be before {after}");
    }

    #[test]
    fn value_returns_inner_datetime() {
        let timestamp = Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap();
        let lease_expires_at = OutboxLeaseExpiresAt::from(timestamp.clone());

        assert_eq!(lease_expires_at.value(), timestamp);
    }

    #[test]
    fn conversions_round_trip() {
        let timestamp = Utc.with_ymd_and_hms(2022, 6, 7, 8, 9, 10).unwrap();
        let wrapped: OutboxLeaseExpiresAt = timestamp.clone().into();
        let back: DateTime<Utc> = wrapped.into();

        assert_eq!(back, timestamp);
    }

    #[test]
    fn display_matches_inner_datetime() {
        let timestamp = Utc.with_ymd_and_hms(2030, 12, 31, 23, 59, 59).unwrap();
        let lease_expires_at = OutboxLeaseExpiresAt::from(timestamp.clone());

        assert_eq!(lease_expires_at.to_string(), timestamp.to_string());
    }

    #[test]
    fn from_now_adds_duration_to_now() {
        let backoff = OutboxLeaseDuration::from(Duration::seconds(30));
        let before = Utc::now();
        let lease_expires_at = OutboxLeaseExpiresAt::from_now(backoff);
        let after = Utc::now();

        let value = lease_expires_at.value();
        let duration = backoff.value();
        let min_expected = before + duration;
        let max_expected = after + duration;

        assert!(
            value >= min_expected,
            "expected {value} to be after {min_expected}"
        );
        assert!(
            value <= max_expected,
            "expected {value} to be before {max_expected}"
        );
    }
}
