use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An opaque resource ID, shared by resource objects and relationship linkage.
///
/// Composite IDs must use an unambiguous encoding.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelId(String);

impl ReadModelId {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<String> for ReadModelId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ReadModelId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<Uuid> for ReadModelId {
    fn from(value: Uuid) -> Self {
        Self(value.to_string())
    }
}

impl AsRef<str> for ReadModelId {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for ReadModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}
