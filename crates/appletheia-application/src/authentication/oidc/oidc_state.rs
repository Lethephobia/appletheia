use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OidcState(String);

impl OidcState {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Default for OidcState {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for OidcState {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(value)?;
        Ok(Self(uuid.to_string()))
    }
}

impl TryFrom<String> for OidcState {
    type Error = uuid::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uuid = Uuid::parse_str(&value)?;
        Ok(Self(uuid.to_string()))
    }
}

impl From<Uuid> for OidcState {
    fn from(value: Uuid) -> Self {
        Self(value.to_string())
    }
}
