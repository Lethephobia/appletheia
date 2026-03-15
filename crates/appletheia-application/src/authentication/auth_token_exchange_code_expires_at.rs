use std::{fmt, fmt::Display};

use chrono::{DateTime, Utc};

use super::AuthTokenExchangeCodeExpiresIn;

/// Represents when an auth token exchange code record expires.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct AuthTokenExchangeCodeExpiresAt(DateTime<Utc>);

impl AuthTokenExchangeCodeExpiresAt {
    /// Creates an expiration timestamp from the current time and the provided lifetime.
    pub fn from_now(expires_in: AuthTokenExchangeCodeExpiresIn) -> Self {
        Self(Utc::now() + expires_in.value())
    }

    /// Returns the wrapped timestamp.
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl From<DateTime<Utc>> for AuthTokenExchangeCodeExpiresAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<AuthTokenExchangeCodeExpiresAt> for DateTime<Utc> {
    fn from(value: AuthTokenExchangeCodeExpiresAt) -> Self {
        value.0
    }
}

impl Display for AuthTokenExchangeCodeExpiresAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
