use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the current UTC date and time used to evaluate domain rules.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CurrentDateTime(DateTime<Utc>);

impl CurrentDateTime {
    /// Creates a new current date-time value.
    pub fn new() -> Self {
        Self(Utc::now())
    }

    /// Returns the underlying UTC timestamp.
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl From<DateTime<Utc>> for CurrentDateTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<CurrentDateTime> for DateTime<Utc> {
    fn from(value: CurrentDateTime) -> Self {
        value.0
    }
}

impl Display for CurrentDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, TimeZone, Utc};

    use super::CurrentDateTime;

    #[test]
    fn value_returns_inner_datetime() {
        let timestamp = Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap();
        let now = CurrentDateTime::from(timestamp);

        assert_eq!(now.value(), timestamp);
    }

    #[test]
    fn conversions_round_trip() {
        let timestamp = Utc.with_ymd_and_hms(2022, 6, 7, 8, 9, 10).unwrap();
        let now: CurrentDateTime = timestamp.into();
        let back_into_datetime: DateTime<Utc> = now.into();

        assert_eq!(back_into_datetime, timestamp);
    }

    #[test]
    fn new_uses_current_utc_datetime() {
        let before = Utc::now();
        let now = CurrentDateTime::new();
        let after = Utc::now();

        assert!(before <= now.value());
        assert!(now.value() <= after);
    }

    #[test]
    fn display_matches_inner_datetime() {
        let timestamp = Utc.with_ymd_and_hms(2030, 12, 31, 23, 59, 59).unwrap();
        let now = CurrentDateTime::from(timestamp);

        assert_eq!(now.to_string(), timestamp.to_string());
    }
}
