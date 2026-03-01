use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcIdTokenExpiresAt(DateTime<Utc>);

impl OidcIdTokenExpiresAt {
    pub fn new(value: DateTime<Utc>) -> Self {
        Self(value)
    }

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}
