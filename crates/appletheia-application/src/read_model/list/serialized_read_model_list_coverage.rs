use serde::{Deserialize, Serialize, de::DeserializeOwned};

use super::SerializedReadModelListCoverageError;

/// Stores an application-defined list coverage in transport-neutral JSON form.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "serde_json::Value", into = "serde_json::Value")]
pub struct SerializedReadModelListCoverage(serde_json::Value);

impl SerializedReadModelListCoverage {
    /// Serializes a typed list coverage.
    pub fn try_from_typed<T>(coverage: &T) -> Result<Self, SerializedReadModelListCoverageError>
    where
        T: Serialize,
    {
        let value = serde_json::to_value(coverage)?;
        Self::try_from(value)
    }

    /// Deserializes the application-defined coverage type.
    pub fn try_into_typed<T>(&self) -> Result<T, SerializedReadModelListCoverageError>
    where
        T: DeserializeOwned,
    {
        serde_json::from_value(self.0.clone()).map_err(Into::into)
    }

    /// Returns the serialized coverage.
    pub fn value(&self) -> &serde_json::Value {
        &self.0
    }
}

impl TryFrom<serde_json::Value> for SerializedReadModelListCoverage {
    type Error = SerializedReadModelListCoverageError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        if value.is_null() {
            return Err(SerializedReadModelListCoverageError::NullCoverage);
        }
        Ok(Self(value))
    }
}

impl From<SerializedReadModelListCoverage> for serde_json::Value {
    fn from(value: SerializedReadModelListCoverage) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_rejects_null() {
        let result = serde_json::from_str::<SerializedReadModelListCoverage>("null");

        assert!(result.is_err());
    }
}
