use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::EventPayloadOwnedError;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventPayloadOwned(serde_json::Value);

impl EventPayloadOwned {
    pub fn value(&self) -> &serde_json::Value {
        &self.0
    }

    fn validate(value: &serde_json::Value) -> Result<(), EventPayloadOwnedError> {
        if value.is_null() {
            return Err(EventPayloadOwnedError::NullPayload);
        }
        Ok(())
    }
}

impl TryFrom<serde_json::Value> for EventPayloadOwned {
    type Error = EventPayloadOwnedError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

impl FromStr for EventPayloadOwned {
    type Err = EventPayloadOwnedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = serde_json::from_str(s)?;
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for EventPayloadOwned {
    type Error = EventPayloadOwnedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for EventPayloadOwned {
    type Error = EventPayloadOwnedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let json = serde_json::from_str::<serde_json::Value>(&value)?;
        Self::validate(&json)?;
        Ok(Self(json))
    }
}

impl Display for EventPayloadOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_null() {
        let err = EventPayloadOwned::try_from(serde_json::Value::Null)
            .expect_err("null should be rejected");
        assert!(matches!(err, EventPayloadOwnedError::NullPayload));
    }

    #[test]
    fn accepts_json_object() {
        let value = serde_json::json!({ "name": "apple" });
        let owned = EventPayloadOwned::try_from(value.clone()).expect("valid");
        assert_eq!(owned.value(), &value);
    }

    #[test]
    fn parses_from_str() {
        let owned: EventPayloadOwned = r#"{"name":"banana"}"#.parse().unwrap();
        assert_eq!(owned.value(), &serde_json::json!({ "name": "banana" }));
    }

    #[test]
    fn detects_invalid_json() {
        let err = EventPayloadOwned::try_from("not-json").expect_err("invalid json");
        assert!(matches!(err, EventPayloadOwnedError::Json { .. }));
    }
}
