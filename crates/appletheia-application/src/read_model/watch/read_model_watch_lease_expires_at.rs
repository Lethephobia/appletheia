use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

use super::ReadModelWatchLeaseDuration;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ReadModelWatchLeaseExpiresAt(DateTime<Utc>);

impl ReadModelWatchLeaseExpiresAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn is_expired_at(&self, now: DateTime<Utc>) -> bool {
        self.0 <= now
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn from_now(lease_duration: ReadModelWatchLeaseDuration) -> Self {
        Self(Utc::now() + lease_duration.value())
    }
}

impl Default for ReadModelWatchLeaseExpiresAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for ReadModelWatchLeaseExpiresAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<ReadModelWatchLeaseExpiresAt> for DateTime<Utc> {
    fn from(value: ReadModelWatchLeaseExpiresAt) -> Self {
        value.0
    }
}

impl Display for ReadModelWatchLeaseExpiresAt {
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
        let expires_at = ReadModelWatchLeaseExpiresAt::from(now);
        assert!(!expires_at.is_expired_at(now - Duration::nanoseconds(1)));
        assert!(expires_at.is_expired_at(now));
        assert!(expires_at.is_expired_at(now + Duration::nanoseconds(1)));
    }
}
