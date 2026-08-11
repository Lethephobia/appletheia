use std::{fmt, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use super::SerializedCommandError;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SerializedCommand(serde_json::Value);

impl SerializedCommand {
    pub fn new(value: serde_json::Value) -> Result<Self, SerializedCommandError> {
        Self::validate(&value)?;
        Ok(Self(value))
    }

    pub fn value(&self) -> &serde_json::Value {
        &self.0
    }

    fn validate(value: &serde_json::Value) -> Result<(), SerializedCommandError> {
        if value.is_null() {
            return Err(SerializedCommandError::NullPayload);
        }
        Ok(())
    }
}

impl TryFrom<serde_json::Value> for SerializedCommand {
    type Error = SerializedCommandError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Display for SerializedCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SerializedCommand {
    type Err = SerializedCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = serde_json::from_str(s)?;
        Self::new(value)
    }
}

impl TryFrom<&str> for SerializedCommand {
    type Error = SerializedCommandError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for SerializedCommand {
    type Error = SerializedCommandError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let json = serde_json::from_str::<serde_json::Value>(&value)?;
        Self::new(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_null() {
        let err = SerializedCommand::try_from(serde_json::Value::Null).expect_err("null rejected");
        assert!(matches!(err, SerializedCommandError::NullPayload));
    }

    #[test]
    fn accepts_json_object() {
        let value = serde_json::json!({ "name": "apple" });
        let command = SerializedCommand::try_from(value.clone()).expect("valid");
        assert_eq!(command.value(), &value);
    }

    #[test]
    fn parses_from_str() {
        let command: SerializedCommand = r#"{"name":"banana"}"#.parse().unwrap();
        assert_eq!(command.value(), &serde_json::json!({ "name": "banana" }));
    }

    #[test]
    fn detects_invalid_json() {
        let err = SerializedCommand::try_from("not-json").expect_err("invalid json");
        assert!(matches!(err, SerializedCommandError::Json { .. }));
    }
}
