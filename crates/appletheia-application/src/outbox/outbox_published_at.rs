use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct OutboxPublishedAt(DateTime<Utc>);

impl OutboxPublishedAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for OutboxPublishedAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for OutboxPublishedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<OutboxPublishedAt> for DateTime<Utc> {
    fn from(value: OutboxPublishedAt) -> Self {
        value.0
    }
}

impl Display for OutboxPublishedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn new_produces_timestamp_close_to_now() {
        let before = Utc::now();
        let published_at = OutboxPublishedAt::now();
        let after = Utc::now();

        let published_at = published_at.value();
        assert!(
            published_at >= before,
            "expected {published_at} to be after {before}"
        );
        assert!(
            published_at <= after,
            "expected {published_at} to be before {after}"
        );
    }

    #[test]
    fn value_returns_inner_datetime() {
        let timestamp = Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap();
        let published_at = OutboxPublishedAt::from(timestamp.clone());

        assert_eq!(published_at.value(), timestamp);
    }

    #[test]
    fn conversions_round_trip() {
        let timestamp = Utc.with_ymd_and_hms(2022, 6, 7, 8, 9, 10).unwrap();
        let published_at: OutboxPublishedAt = timestamp.clone().into();
        let back_into_datetime: DateTime<Utc> = published_at.into();

        assert_eq!(back_into_datetime, timestamp);
    }

    #[test]
    fn display_matches_inner_datetime() {
        let timestamp = Utc.with_ymd_and_hms(2030, 12, 31, 23, 59, 59).unwrap();
        let published_at = OutboxPublishedAt::from(timestamp.clone());

        assert_eq!(published_at.to_string(), timestamp.to_string());
    }
}
