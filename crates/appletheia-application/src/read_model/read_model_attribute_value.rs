use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The contained attribute value, without its typed enum's variant tag.
/// Scalars, objects, arrays, and null are all preserved.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelAttributeValue(Value);

impl ReadModelAttributeValue {
    pub fn value(&self) -> &Value {
        &self.0
    }
}

impl From<Value> for ReadModelAttributeValue {
    fn from(value: Value) -> Self {
        Self(value)
    }
}
