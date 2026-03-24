use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents when an OIDC login attempt started.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcLoginAttemptStartedAt(DateTime<Utc>);

impl OidcLoginAttemptStartedAt {
    /// Returns the current timestamp.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Returns the wrapped timestamp.
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for OidcLoginAttemptStartedAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for OidcLoginAttemptStartedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<OidcLoginAttemptStartedAt> for DateTime<Utc> {
    fn from(value: OidcLoginAttemptStartedAt) -> Self {
        value.0
    }
}

impl Display for OidcLoginAttemptStartedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
