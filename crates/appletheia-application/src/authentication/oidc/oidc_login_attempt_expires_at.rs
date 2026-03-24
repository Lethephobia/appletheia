use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{OidcLoginAttemptCreatedAt, OidcLoginAttemptExpiresIn};

/// Represents when an OIDC login attempt expires.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcLoginAttemptExpiresAt(DateTime<Utc>);

impl OidcLoginAttemptExpiresAt {
    /// Creates an expiration timestamp from `created_at` and the provided lifetime.
    pub fn from_created_at(
        created_at: OidcLoginAttemptCreatedAt,
        expires_in: OidcLoginAttemptExpiresIn,
    ) -> Self {
        Self(created_at.value() + expires_in.value())
    }

    /// Returns the wrapped timestamp.
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl From<DateTime<Utc>> for OidcLoginAttemptExpiresAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<OidcLoginAttemptExpiresAt> for DateTime<Utc> {
    fn from(value: OidcLoginAttemptExpiresAt) -> Self {
        value.0
    }
}

impl Display for OidcLoginAttemptExpiresAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
