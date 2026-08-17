use serde::{Deserialize, Serialize};

use super::SerializedReadModelPartError;

/// Stores a complete read model part in transport-neutral JSON form.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "serde_json::Value", into = "serde_json::Value")]
pub struct SerializedReadModelPart(serde_json::Value);

impl SerializedReadModelPart {
    /// Returns the serialized part value.
    pub fn value(&self) -> &serde_json::Value {
        &self.0
    }
}

impl TryFrom<serde_json::Value> for SerializedReadModelPart {
    type Error = SerializedReadModelPartError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        if value.is_null() {
            return Err(SerializedReadModelPartError::NullPart);
        }

        Ok(Self(value))
    }
}

impl From<SerializedReadModelPart> for serde_json::Value {
    fn from(value: SerializedReadModelPart) -> Self {
        value.0
    }
}
