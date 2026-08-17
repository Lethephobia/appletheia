use serde::{Deserialize, Serialize, de::DeserializeOwned};

use super::SerializedReadModelListQueryError;

/// Stores an application-defined list query in transport-neutral JSON form.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "serde_json::Value", into = "serde_json::Value")]
pub struct SerializedReadModelListQuery(serde_json::Value);

impl SerializedReadModelListQuery {
    /// Serializes a typed list query.
    pub fn try_from_typed<T>(query: &T) -> Result<Self, SerializedReadModelListQueryError>
    where
        T: Serialize,
    {
        let value = serde_json::to_value(query)?;
        Self::try_from(value)
    }

    /// Deserializes the application-defined query type.
    pub fn try_into_typed<T>(&self) -> Result<T, SerializedReadModelListQueryError>
    where
        T: DeserializeOwned,
    {
        serde_json::from_value(self.0.clone()).map_err(Into::into)
    }

    /// Returns the serialized query.
    pub fn value(&self) -> &serde_json::Value {
        &self.0
    }
}

impl TryFrom<serde_json::Value> for SerializedReadModelListQuery {
    type Error = SerializedReadModelListQueryError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        if value.is_null() {
            return Err(SerializedReadModelListQueryError::NullQuery);
        }
        Ok(Self(value))
    }
}

impl From<SerializedReadModelListQuery> for serde_json::Value {
    fn from(value: SerializedReadModelListQuery) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_rejects_null() {
        let result = serde_json::from_str::<SerializedReadModelListQuery>("null");

        assert!(result.is_err());
    }
}
