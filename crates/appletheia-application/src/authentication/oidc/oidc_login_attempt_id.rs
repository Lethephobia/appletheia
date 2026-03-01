use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcLoginAttemptId(Uuid);

impl OidcLoginAttemptId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for OidcLoginAttemptId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for OidcLoginAttemptId {
    fn from(value: Uuid) -> Self {
        Self::from_uuid(value)
    }
}

impl From<OidcLoginAttemptId> for Uuid {
    fn from(value: OidcLoginAttemptId) -> Self {
        value.value()
    }
}
