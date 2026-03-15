use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

/// Represents when an auth token exchange code record was created.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct AuthTokenExchangeCodeCreatedAt(DateTime<Utc>);

impl AuthTokenExchangeCodeCreatedAt {
    /// Returns the current timestamp.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Returns the wrapped timestamp.
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for AuthTokenExchangeCodeCreatedAt {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for AuthTokenExchangeCodeCreatedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<AuthTokenExchangeCodeCreatedAt> for DateTime<Utc> {
    fn from(value: AuthTokenExchangeCodeCreatedAt) -> Self {
        value.0
    }
}

impl Display for AuthTokenExchangeCodeCreatedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
