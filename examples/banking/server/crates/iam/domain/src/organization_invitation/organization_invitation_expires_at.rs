use std::{fmt, fmt::Display};

use banking_shared_kernel_domain::timestamps::CurrentDateTime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents when an organization invitation expires.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrganizationInvitationExpiresAt(DateTime<Utc>);

impl OrganizationInvitationExpiresAt {
    /// Creates a new expiration timestamp.
    pub fn new(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    /// Returns the underlying UTC timestamp.
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }

    /// Returns whether the invitation is expired at the provided time.
    pub fn is_expired(&self, now: CurrentDateTime) -> bool {
        self.0 <= now.value()
    }
}

impl From<DateTime<Utc>> for OrganizationInvitationExpiresAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<OrganizationInvitationExpiresAt> for DateTime<Utc> {
    fn from(value: OrganizationInvitationExpiresAt) -> Self {
        value.0
    }
}

impl Display for OrganizationInvitationExpiresAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use banking_shared_kernel_domain::timestamps::CurrentDateTime;
    use chrono::{DateTime, TimeZone, Utc};

    use super::OrganizationInvitationExpiresAt;

    #[test]
    fn value_returns_inner_datetime() {
        let timestamp = Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap();
        let expires_at = OrganizationInvitationExpiresAt::from(timestamp);

        assert_eq!(expires_at.value(), timestamp);
    }

    #[test]
    fn conversions_round_trip() {
        let timestamp = Utc.with_ymd_and_hms(2022, 6, 7, 8, 9, 10).unwrap();
        let expires_at: OrganizationInvitationExpiresAt = timestamp.into();
        let back_into_datetime: DateTime<Utc> = expires_at.into();

        assert_eq!(back_into_datetime, timestamp);
    }

    #[test]
    fn display_matches_inner_datetime() {
        let timestamp = Utc.with_ymd_and_hms(2030, 12, 31, 23, 59, 59).unwrap();
        let expires_at = OrganizationInvitationExpiresAt::from(timestamp);

        assert_eq!(expires_at.to_string(), timestamp.to_string());
    }

    #[test]
    fn is_expired_compares_against_provided_time() {
        let expires_at = OrganizationInvitationExpiresAt::from(
            Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap(),
        );

        assert!(!expires_at.is_expired(CurrentDateTime::from(
            Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 4).unwrap(),
        )));
        assert!(expires_at.is_expired(CurrentDateTime::from(
            Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap(),
        )));
    }
}
