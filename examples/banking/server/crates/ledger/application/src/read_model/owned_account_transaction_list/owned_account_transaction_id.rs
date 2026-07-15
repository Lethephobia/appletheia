use std::{
    convert::Infallible,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifies an owned account transaction read model row.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OwnedAccountTransactionId(Uuid);

impl OwnedAccountTransactionId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }

    pub fn try_from_uuid(value: Uuid) -> Result<Self, Infallible> {
        Ok(Self(value))
    }
}

impl Default for OwnedAccountTransactionId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for OwnedAccountTransactionId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<OwnedAccountTransactionId> for Uuid {
    fn from(value: OwnedAccountTransactionId) -> Self {
        value.value()
    }
}

impl Display for OwnedAccountTransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
