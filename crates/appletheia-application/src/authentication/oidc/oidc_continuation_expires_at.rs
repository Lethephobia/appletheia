use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::OidcLoginAttemptExpiresAt;

/// Represents when an OIDC continuation expires.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcContinuationExpiresAt(DateTime<Utc>);

impl OidcContinuationExpiresAt {
    /// Returns the wrapped timestamp.
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl From<DateTime<Utc>> for OidcContinuationExpiresAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<OidcContinuationExpiresAt> for DateTime<Utc> {
    fn from(value: OidcContinuationExpiresAt) -> Self {
        value.0
    }
}

impl From<OidcLoginAttemptExpiresAt> for OidcContinuationExpiresAt {
    fn from(value: OidcLoginAttemptExpiresAt) -> Self {
        Self(value.value())
    }
}

impl Display for OidcContinuationExpiresAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
