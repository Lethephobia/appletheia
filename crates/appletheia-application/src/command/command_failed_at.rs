use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the timestamp at which a command became terminally failed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CommandFailedAt(DateTime<Utc>);

impl CommandFailedAt {
    /// Creates a command failure timestamp using the current UTC time.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Returns the underlying UTC timestamp.
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for CommandFailedAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for CommandFailedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<CommandFailedAt> for DateTime<Utc> {
    fn from(value: CommandFailedAt) -> Self {
        value.value()
    }
}

impl Display for CommandFailedAt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn now_produces_timestamp_close_to_current_time() {
        let before = Utc::now();
        let failed_at = CommandFailedAt::now();
        let after = Utc::now();

        assert!(failed_at.value() >= before);
        assert!(failed_at.value() <= after);
    }

    #[test]
    fn conversions_round_trip() {
        let timestamp = Utc.with_ymd_and_hms(2026, 9, 5, 1, 2, 3).unwrap();
        let failed_at = CommandFailedAt::from(timestamp);
        let restored: DateTime<Utc> = failed_at.into();

        assert_eq!(restored, timestamp);
    }

    #[test]
    fn display_formats_underlying_timestamp() {
        let timestamp = Utc.with_ymd_and_hms(2026, 9, 5, 1, 2, 3).unwrap();
        let failed_at = CommandFailedAt::from(timestamp);

        assert_eq!(failed_at.to_string(), timestamp.to_string());
    }
}
