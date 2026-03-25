use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents when an OIDC login attempt was consumed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcLoginAttemptConsumedAt(DateTime<Utc>);

impl OidcLoginAttemptConsumedAt {
    /// Returns the current timestamp.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Returns the wrapped timestamp.
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for OidcLoginAttemptConsumedAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for OidcLoginAttemptConsumedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<OidcLoginAttemptConsumedAt> for DateTime<Utc> {
    fn from(value: OidcLoginAttemptConsumedAt) -> Self {
        value.0
    }
}

impl Display for OidcLoginAttemptConsumedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
