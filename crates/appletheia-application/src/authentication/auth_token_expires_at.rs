use std::{fmt, fmt::Display};

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use super::{AuthTokenExpiresAtError, AuthTokenExpiresIn};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AuthTokenExpiresAt(DateTime<Utc>);

impl AuthTokenExpiresAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn from_now(expires_in: AuthTokenExpiresIn) -> Self {
        Self(Utc::now() + expires_in.value())
    }

    pub fn to_unix_timestamp_seconds(&self) -> Result<u64, AuthTokenExpiresAtError> {
        let seconds = self.0.timestamp();
        u64::try_from(seconds).map_err(|_| AuthTokenExpiresAtError::BeforeUnixEpoch)
    }

    pub fn from_unix_timestamp_seconds(seconds: u64) -> Result<Self, AuthTokenExpiresAtError> {
        let seconds_i64 = i64::try_from(seconds)
            .map_err(|_| AuthTokenExpiresAtError::InvalidUnixTimestampSeconds { seconds })?;
        let value = Utc
            .timestamp_opt(seconds_i64, 0)
            .single()
            .ok_or(AuthTokenExpiresAtError::InvalidUnixTimestampSeconds { seconds })?;
        Ok(Self(value))
    }
}

impl Default for AuthTokenExpiresAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for AuthTokenExpiresAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<AuthTokenExpiresAt> for DateTime<Utc> {
    fn from(value: AuthTokenExpiresAt) -> Self {
        value.0
    }
}

impl Display for AuthTokenExpiresAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::AuthTokenExpiresAt;
    use crate::authentication::AuthTokenExpiresAtError;

    #[test]
    fn to_unix_timestamp_seconds_returns_seconds_since_epoch() {
        let expires_at = AuthTokenExpiresAt::from(
            Utc.with_ymd_and_hms(2026, 3, 13, 12, 0, 0)
                .single()
                .expect("valid timestamp"),
        );

        let seconds = expires_at
            .to_unix_timestamp_seconds()
            .expect("timestamp should be valid");

        assert_eq!(seconds, 1_773_403_200);
    }

    #[test]
    fn to_unix_timestamp_seconds_rejects_timestamps_before_epoch() {
        let expires_at = AuthTokenExpiresAt::from(
            Utc.with_ymd_and_hms(1969, 12, 31, 23, 59, 59)
                .single()
                .expect("valid timestamp"),
        );

        let error = expires_at
            .to_unix_timestamp_seconds()
            .expect_err("timestamp should be rejected");

        assert_eq!(error, AuthTokenExpiresAtError::BeforeUnixEpoch);
    }

    #[test]
    fn from_unix_timestamp_seconds_restores_timestamp() {
        let expires_at = AuthTokenExpiresAt::from_unix_timestamp_seconds(1_773_403_200)
            .expect("timestamp should be valid");

        assert_eq!(
            expires_at.value(),
            Utc.with_ymd_and_hms(2026, 3, 13, 12, 0, 0)
                .single()
                .expect("valid timestamp")
        );
    }

    #[test]
    fn from_unix_timestamp_seconds_rejects_out_of_range_values() {
        let seconds = u64::MAX;

        let error = AuthTokenExpiresAt::from_unix_timestamp_seconds(seconds)
            .expect_err("timestamp should be rejected");

        assert_eq!(
            error,
            AuthTokenExpiresAtError::InvalidUnixTimestampSeconds { seconds }
        );
    }
}
