use std::{fmt, fmt::Display};

use banking_shared_kernel_domain::timestamps::CurrentDateTime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents when an currency registrar invitation expires.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CurrencyRegistrarInvitationExpiresAt(DateTime<Utc>);

impl CurrencyRegistrarInvitationExpiresAt {
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

impl From<DateTime<Utc>> for CurrencyRegistrarInvitationExpiresAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<CurrencyRegistrarInvitationExpiresAt> for DateTime<Utc> {
    fn from(value: CurrencyRegistrarInvitationExpiresAt) -> Self {
        value.0
    }
}

impl Display for CurrencyRegistrarInvitationExpiresAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
