use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AuthTokenId(Uuid);

impl AuthTokenId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for AuthTokenId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AuthTokenId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl From<Uuid> for AuthTokenId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<AuthTokenId> for Uuid {
    fn from(value: AuthTokenId) -> Self {
        value.0
    }
}
