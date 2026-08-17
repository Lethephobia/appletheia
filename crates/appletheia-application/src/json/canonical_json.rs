use std::hash::{Hash, Hasher};

use serde::Serialize;

/// Stores a JSON value with recursively sorted object keys.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalJson(String);

impl CanonicalJson {
    /// Canonicalizes a JSON value.
    pub fn from_value(value: &serde_json::Value) -> Self {
        let mut output = String::new();
        Self::write_value(value, &mut output);
        Self(output)
    }

    /// Serializes and canonicalizes a value.
    pub fn try_from_serializable<T>(value: &T) -> Result<Self, serde_json::Error>
    where
        T: Serialize,
    {
        let serialized_value = serde_json::to_value(value)?;
        Ok(Self::from_value(&serialized_value))
    }

    /// Returns the canonical JSON text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the canonical JSON text.
    pub fn into_string(self) -> String {
        self.0
    }

    fn write_value(value: &serde_json::Value, output: &mut String) {
        match value {
            serde_json::Value::Null => output.push_str("null"),
            serde_json::Value::Bool(boolean) => {
                output.push_str(if *boolean { "true" } else { "false" });
            }
            serde_json::Value::Number(number) => output.push_str(&number.to_string()),
            serde_json::Value::String(string) => Self::write_string(string, output),
            serde_json::Value::Array(values) => {
                output.push('[');
                for (index, array_value) in values.iter().enumerate() {
                    if index > 0 {
                        output.push(',');
                    }
                    Self::write_value(array_value, output);
                }
                output.push(']');
            }
            serde_json::Value::Object(object) => {
                output.push('{');
                let mut entries = object.iter().collect::<Vec<_>>();
                entries.sort_unstable_by_key(|(key, _)| *key);
                for (index, (key, object_value)) in entries.into_iter().enumerate() {
                    if index > 0 {
                        output.push(',');
                    }
                    Self::write_string(key, output);
                    output.push(':');
                    Self::write_value(object_value, output);
                }
                output.push('}');
            }
        }
    }

    fn write_string(value: &str, output: &mut String) {
        output.push('"');
        for character in value.chars() {
            match character {
                '"' => output.push_str("\\\""),
                '\\' => output.push_str("\\\\"),
                '\u{08}' => output.push_str("\\b"),
                '\u{0C}' => output.push_str("\\f"),
                '\n' => output.push_str("\\n"),
                '\r' => output.push_str("\\r"),
                '\t' => output.push_str("\\t"),
                character if character.is_control() => {
                    output.push_str(&format!("\\u{:04x}", character as u32));
                }
                character => output.push(character),
            }
        }
        output.push('"');
    }
}

impl Hash for CanonicalJson {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::CanonicalJson;

    #[test]
    fn sorts_object_keys_recursively() {
        let value = serde_json::json!({
            "z": { "y": 1, "x": 2 },
            "a": [ { "d": 3, "c": 4 } ],
        });

        let json = CanonicalJson::from_value(&value);

        assert_eq!(json.as_str(), r#"{"a":[{"c":4,"d":3}],"z":{"x":2,"y":1}}"#);
    }
}
