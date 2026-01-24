use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::SerializedEventPayloadError;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SerializedEventPayload(serde_json::Value);

impl SerializedEventPayload {
    pub fn value(&self) -> &serde_json::Value {
        &self.0
    }

    fn validate(value: &serde_json::Value) -> Result<(), SerializedEventPayloadError> {
        if value.is_null() {
            return Err(SerializedEventPayloadError::NullPayload);
        }
        Ok(())
    }
}

impl TryFrom<serde_json::Value> for SerializedEventPayload {
    type Error = SerializedEventPayloadError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

impl FromStr for SerializedEventPayload {
    type Err = SerializedEventPayloadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = serde_json::from_str(s)?;
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for SerializedEventPayload {
    type Error = SerializedEventPayloadError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for SerializedEventPayload {
    type Error = SerializedEventPayloadError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let json = serde_json::from_str::<serde_json::Value>(&value)?;
        Self::validate(&json)?;
        Ok(Self(json))
    }
}

impl Display for SerializedEventPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_null() {
        let err = SerializedEventPayload::try_from(serde_json::Value::Null)
            .expect_err("null should be rejected");
        assert!(matches!(err, SerializedEventPayloadError::NullPayload));
    }

    #[test]
    fn accepts_json_object() {
        let value = serde_json::json!({ "name": "apple" });
        let payload = SerializedEventPayload::try_from(value.clone()).expect("valid");
        assert_eq!(payload.value(), &value);
    }

    #[test]
    fn parses_from_str() {
        let payload: SerializedEventPayload = r#"{"name":"banana"}"#.parse().unwrap();
        assert_eq!(payload.value(), &serde_json::json!({ "name": "banana" }));
    }

    #[test]
    fn detects_invalid_json() {
        let err = SerializedEventPayload::try_from("not-json").expect_err("invalid json");
        assert!(matches!(err, SerializedEventPayloadError::Json { .. }));
    }
}
