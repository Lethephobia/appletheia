use std::{fmt, fmt::Display};

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use super::AuthTokenIssuedAtError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AuthTokenIssuedAt(DateTime<Utc>);

impl AuthTokenIssuedAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn to_unix_timestamp_seconds(&self) -> Result<u64, AuthTokenIssuedAtError> {
        let seconds = self.0.timestamp();
        u64::try_from(seconds).map_err(|_| AuthTokenIssuedAtError::BeforeUnixEpoch)
    }

    pub fn from_unix_timestamp_seconds(seconds: u64) -> Result<Self, AuthTokenIssuedAtError> {
        let seconds_i64 = i64::try_from(seconds)
            .map_err(|_| AuthTokenIssuedAtError::InvalidUnixTimestampSeconds { seconds })?;
        let value = Utc
            .timestamp_opt(seconds_i64, 0)
            .single()
            .ok_or(AuthTokenIssuedAtError::InvalidUnixTimestampSeconds { seconds })?;
        Ok(Self(value))
    }
}

impl Default for AuthTokenIssuedAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for AuthTokenIssuedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<AuthTokenIssuedAt> for DateTime<Utc> {
    fn from(value: AuthTokenIssuedAt) -> Self {
        value.0
    }
}

impl Display for AuthTokenIssuedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::AuthTokenIssuedAt;
    use crate::authentication::AuthTokenIssuedAtError;

    #[test]
    fn to_unix_timestamp_seconds_returns_seconds_since_epoch() {
        let issued_at = AuthTokenIssuedAt::from(
            Utc.with_ymd_and_hms(2026, 3, 13, 12, 0, 0)
                .single()
                .expect("valid timestamp"),
        );

        let seconds = issued_at
            .to_unix_timestamp_seconds()
            .expect("timestamp should be valid");

        assert_eq!(seconds, 1_773_403_200);
    }

    #[test]
    fn to_unix_timestamp_seconds_rejects_timestamps_before_epoch() {
        let issued_at = AuthTokenIssuedAt::from(
            Utc.with_ymd_and_hms(1969, 12, 31, 23, 59, 59)
                .single()
                .expect("valid timestamp"),
        );

        let error = issued_at
            .to_unix_timestamp_seconds()
            .expect_err("timestamp should be rejected");

        assert_eq!(error, AuthTokenIssuedAtError::BeforeUnixEpoch);
    }

    #[test]
    fn from_unix_timestamp_seconds_restores_timestamp() {
        let issued_at = AuthTokenIssuedAt::from_unix_timestamp_seconds(1_773_403_200)
            .expect("timestamp should be valid");

        assert_eq!(
            issued_at.value(),
            Utc.with_ymd_and_hms(2026, 3, 13, 12, 0, 0)
                .single()
                .expect("valid timestamp")
        );
    }

    #[test]
    fn from_unix_timestamp_seconds_rejects_out_of_range_values() {
        let seconds = u64::MAX;

        let error = AuthTokenIssuedAt::from_unix_timestamp_seconds(seconds)
            .expect_err("timestamp should be rejected");

        assert_eq!(
            error,
            AuthTokenIssuedAtError::InvalidUnixTimestampSeconds { seconds }
        );
    }
}
