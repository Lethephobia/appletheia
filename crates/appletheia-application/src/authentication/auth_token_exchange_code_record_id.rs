use std::{fmt, fmt::Display};

use uuid::Uuid;

/// Identifies a persisted auth token exchange code record.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct AuthTokenExchangeCodeRecordId(Uuid);

impl AuthTokenExchangeCodeRecordId {
    /// Creates a new record identifier.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the wrapped identifier value.
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for AuthTokenExchangeCodeRecordId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for AuthTokenExchangeCodeRecordId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<AuthTokenExchangeCodeRecordId> for Uuid {
    fn from(value: AuthTokenExchangeCodeRecordId) -> Self {
        value.0
    }
}

impl Display for AuthTokenExchangeCodeRecordId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
