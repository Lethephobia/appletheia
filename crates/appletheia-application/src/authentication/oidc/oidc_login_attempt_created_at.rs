use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents when an OIDC login attempt was created.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcLoginAttemptCreatedAt(DateTime<Utc>);

impl OidcLoginAttemptCreatedAt {
    /// Returns the current timestamp.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Returns the wrapped timestamp.
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for OidcLoginAttemptCreatedAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for OidcLoginAttemptCreatedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<OidcLoginAttemptCreatedAt> for DateTime<Utc> {
    fn from(value: OidcLoginAttemptCreatedAt) -> Self {
        value.0
    }
}

impl Display for OidcLoginAttemptCreatedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
