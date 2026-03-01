use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HttpResourceEntityTag(String);

impl HttpResourceEntityTag {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn from_header_str(value: &str) -> Self {
        Self(value.to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
