use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Contains one complete client-facing read-model value.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SerializedReadModelSnapshot(Value);

impl SerializedReadModelSnapshot {
    /// Returns the JSON value.
    pub fn value(&self) -> &Value {
        &self.0
    }
}

impl From<Value> for SerializedReadModelSnapshot {
    fn from(value: Value) -> Self {
        Self(value)
    }
}
