use chrono::{DateTime, Utc};

use super::{ReadModelWatchIndexExpiresAtError, ReadModelWatchLeaseDuration};

/// Records when an endpoint's shared-index registration expires.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ReadModelWatchIndexExpiresAt(DateTime<Utc>);

impl ReadModelWatchIndexExpiresAt {
    pub fn new(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    pub fn after(
        now: DateTime<Utc>,
        lease_duration: ReadModelWatchLeaseDuration,
    ) -> Result<Self, ReadModelWatchIndexExpiresAtError> {
        now.checked_add_signed(lease_duration.value())
            .map(Self)
            .ok_or(ReadModelWatchIndexExpiresAtError::Overflow)
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn is_expired_at(&self, now: DateTime<Utc>) -> bool {
        self.0 <= now
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn expires_exactly_at_the_deadline() {
        let now = Utc::now();
        let duration = ReadModelWatchLeaseDuration::new(Duration::seconds(60)).expect("duration");
        let expires_at = ReadModelWatchIndexExpiresAt::after(now, duration).expect("expiration");
        assert!(!expires_at.is_expired_at(now));
        assert!(!expires_at.is_expired_at(expires_at.value() - Duration::nanoseconds(1)));
        assert!(expires_at.is_expired_at(expires_at.value()));
    }

    #[test]
    fn rejects_timestamp_overflow() {
        let duration = ReadModelWatchLeaseDuration::new(Duration::seconds(1)).expect("duration");
        assert!(matches!(
            ReadModelWatchIndexExpiresAt::after(DateTime::<Utc>::MAX_UTC, duration),
            Err(ReadModelWatchIndexExpiresAtError::Overflow),
        ));
    }
}
