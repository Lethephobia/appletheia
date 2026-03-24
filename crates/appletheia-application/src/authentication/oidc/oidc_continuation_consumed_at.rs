use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents when an OIDC continuation was consumed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcContinuationConsumedAt(DateTime<Utc>);

impl OidcContinuationConsumedAt {
    /// Returns the current timestamp.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Returns the wrapped timestamp.
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for OidcContinuationConsumedAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for OidcContinuationConsumedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<OidcContinuationConsumedAt> for DateTime<Utc> {
    fn from(value: OidcContinuationConsumedAt) -> Self {
        value.0
    }
}

impl Display for OidcContinuationConsumedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
