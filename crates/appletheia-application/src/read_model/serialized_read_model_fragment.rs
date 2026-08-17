use serde::{Deserialize, Serialize};

use super::SerializedReadModelFragmentError;

/// Stores a complete fragment value in transport-neutral JSON form.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "serde_json::Value", into = "serde_json::Value")]
pub struct SerializedReadModelFragment(serde_json::Value);

impl SerializedReadModelFragment {
    /// Returns the serialized fragment value.
    pub fn value(&self) -> &serde_json::Value {
        &self.0
    }
}

impl TryFrom<serde_json::Value> for SerializedReadModelFragment {
    type Error = SerializedReadModelFragmentError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        if value.is_null() {
            return Err(SerializedReadModelFragmentError::NullFragment);
        }

        Ok(Self(value))
    }
}

impl From<SerializedReadModelFragment> for serde_json::Value {
    fn from(value: SerializedReadModelFragment) -> Self {
        value.0
    }
}
