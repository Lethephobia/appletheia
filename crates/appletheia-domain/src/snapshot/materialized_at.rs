use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct MaterializedAt(DateTime<Utc>);

impl MaterializedAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for MaterializedAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for MaterializedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<MaterializedAt> for DateTime<Utc> {
    fn from(value: MaterializedAt) -> Self {
        value.0
    }
}

impl Display for MaterializedAt {
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
        let materialized_at = MaterializedAt::now();
        let after = Utc::now();

        let materialized_at = materialized_at.value();
        assert!(
            materialized_at >= before,
            "expected {materialized_at} to be after {before}"
        );
        assert!(
            materialized_at <= after,
            "expected {materialized_at} to be before {after}"
        );
    }

    #[test]
    fn value_returns_inner_datetime() {
        let timestamp = Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap();
        let materialized_at = MaterializedAt::from(timestamp.clone());

        assert_eq!(materialized_at.value(), timestamp);
    }

    #[test]
    fn conversions_round_trip() {
        let timestamp = Utc.with_ymd_and_hms(2022, 6, 7, 8, 9, 10).unwrap();
        let materialized_at: MaterializedAt = timestamp.clone().into();
        let back_into_datetime: DateTime<Utc> = materialized_at.into();

        assert_eq!(back_into_datetime, timestamp);
    }

    #[test]
    fn display_matches_inner_datetime() {
        let timestamp = Utc.with_ymd_and_hms(2030, 12, 31, 23, 59, 59).unwrap();
        let materialized_at = MaterializedAt::from(timestamp.clone());

        assert_eq!(materialized_at.to_string(), timestamp.to_string());
    }
}
