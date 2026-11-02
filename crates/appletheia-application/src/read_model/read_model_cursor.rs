use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

/// An opaque pagination boundary encoded and interpreted by the query implementation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelCursor(String);

impl ReadModelCursor {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<String> for ReadModelCursor {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ReadModelCursor {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl AsRef<str> for ReadModelCursor {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for ReadModelCursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}
