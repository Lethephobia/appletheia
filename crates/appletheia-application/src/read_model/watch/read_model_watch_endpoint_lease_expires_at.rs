use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

use super::ReadModelWatchEndpointLeaseDuration;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ReadModelWatchEndpointLeaseExpiresAt(DateTime<Utc>);

impl ReadModelWatchEndpointLeaseExpiresAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn is_expired_at(&self, now: DateTime<Utc>) -> bool {
        self.0 <= now
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn from_now(lease_duration: ReadModelWatchEndpointLeaseDuration) -> Self {
        Self(Utc::now() + lease_duration.value())
    }
}

impl Default for ReadModelWatchEndpointLeaseExpiresAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for ReadModelWatchEndpointLeaseExpiresAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<ReadModelWatchEndpointLeaseExpiresAt> for DateTime<Utc> {
    fn from(value: ReadModelWatchEndpointLeaseExpiresAt) -> Self {
        value.0
    }
}

impl Display for ReadModelWatchEndpointLeaseExpiresAt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn expires_at_the_deadline() {
        let now = Utc::now();
        let expires_at = ReadModelWatchEndpointLeaseExpiresAt::from(now);
        assert!(!expires_at.is_expired_at(now - Duration::nanoseconds(1)));
        assert!(expires_at.is_expired_at(now));
        assert!(expires_at.is_expired_at(now + Duration::nanoseconds(1)));
    }
}
