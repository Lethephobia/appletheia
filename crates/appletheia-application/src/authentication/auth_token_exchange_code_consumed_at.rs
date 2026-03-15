use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

/// Represents when an auth token exchange code record was consumed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct AuthTokenExchangeCodeConsumedAt(DateTime<Utc>);

impl AuthTokenExchangeCodeConsumedAt {
    /// Returns the current timestamp.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Returns the wrapped timestamp.
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for AuthTokenExchangeCodeConsumedAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for AuthTokenExchangeCodeConsumedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<AuthTokenExchangeCodeConsumedAt> for DateTime<Utc> {
    fn from(value: AuthTokenExchangeCodeConsumedAt) -> Self {
        value.0
    }
}

impl Display for AuthTokenExchangeCodeConsumedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
