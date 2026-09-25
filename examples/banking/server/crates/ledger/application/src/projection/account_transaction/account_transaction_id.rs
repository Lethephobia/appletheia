use std::{
    convert::Infallible,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifies an independently stored account transaction fragment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountTransactionId(Uuid);

impl AccountTransactionId {
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

impl Default for AccountTransactionId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for AccountTransactionId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<AccountTransactionId> for Uuid {
    fn from(value: AccountTransactionId) -> Self {
        value.value()
    }
}

impl Display for AccountTransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
