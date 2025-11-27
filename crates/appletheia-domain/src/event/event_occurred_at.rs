use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EventOccurredAt(DateTime<Utc>);

impl EventOccurredAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for EventOccurredAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for EventOccurredAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<EventOccurredAt> for DateTime<Utc> {
    fn from(value: EventOccurredAt) -> Self {
        value.0
    }
}

impl Display for EventOccurredAt {
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
        let occurred_at = EventOccurredAt::now();
        let after = Utc::now();

        let occurred_at = occurred_at.value();
        assert!(
            occurred_at >= before,
            "expected {occurred_at} to be after {before}"
        );
        assert!(
            occurred_at <= after,
            "expected {occurred_at} to be before {after}"
        );
    }

    #[test]
    fn value_returns_inner_datetime() {
        let timestamp = Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap();
        let occurred_at = EventOccurredAt::from(timestamp.clone());

        assert_eq!(occurred_at.value(), timestamp);
    }

    #[test]
    fn conversions_round_trip() {
        let timestamp = Utc.with_ymd_and_hms(2022, 6, 7, 8, 9, 10).unwrap();
        let occurred_at: EventOccurredAt = timestamp.clone().into();
        let back_into_datetime: DateTime<Utc> = occurred_at.into();

        assert_eq!(back_into_datetime, timestamp);
    }

    #[test]
    fn display_matches_inner_datetime() {
        let timestamp = Utc.with_ymd_and_hms(2030, 12, 31, 23, 59, 59).unwrap();
        let occurred_at = EventOccurredAt::from(timestamp.clone());

        assert_eq!(occurred_at.to_string(), timestamp.to_string());
    }
}
