use std::fmt::{self, Display};

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::ReadModelResourceWindowCursorError;

/// An opaque pagination boundary encoded and interpreted by the query implementation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelResourceWindowCursor(String);

impl ReadModelResourceWindowCursor {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn try_from_json(json: Value) -> Result<Self, ReadModelResourceWindowCursorError> {
        Ok(Self(URL_SAFE_NO_PAD.encode(serde_json::to_vec(&json)?)))
    }

    pub fn try_into_json(self) -> Result<Value, ReadModelResourceWindowCursorError> {
        let json = URL_SAFE_NO_PAD.decode(&self.0)?;
        Ok(serde_json::from_slice(&json)?)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<String> for ReadModelResourceWindowCursor {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ReadModelResourceWindowCursor {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl AsRef<str> for ReadModelResourceWindowCursor {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for ReadModelResourceWindowCursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn round_trips_json_as_url_safe_unpadded_string() {
        for value in [
            json!({"name": "東京 / + =", "id": 42}),
            json!([1, null, true]),
            json!("boundary"),
            json!(42),
            Value::Null,
        ] {
            let cursor = ReadModelResourceWindowCursor::try_from_json(value.clone()).unwrap();
            assert!(
                cursor
                    .value()
                    .bytes()
                    .all(|byte| { byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' })
            );
            let serialized_cursor = serde_json::to_string(&cursor).unwrap();
            let restored: ReadModelResourceWindowCursor =
                serde_json::from_str(&serialized_cursor).unwrap();
            assert_eq!(restored.try_into_json().unwrap(), value);
        }
    }

    #[test]
    fn reports_invalid_base64_and_json() {
        assert!(matches!(
            ReadModelResourceWindowCursor::from("!").try_into_json(),
            Err(ReadModelResourceWindowCursorError::Base64(_))
        ));
        for bytes in [b"not json".as_slice(), &[0xff]] {
            let invalid_json = ReadModelResourceWindowCursor::new(URL_SAFE_NO_PAD.encode(bytes));
            assert!(matches!(
                invalid_json.try_into_json(),
                Err(ReadModelResourceWindowCursorError::Json(_))
            ));
        }
    }
}
