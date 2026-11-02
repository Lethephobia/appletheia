use std::fmt::{self, Display};

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::ReadModelWindowCursorError;

/// An opaque pagination boundary encoded and interpreted by the query implementation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadModelWindowCursor(String);

impl ReadModelWindowCursor {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn try_from_serializable<T>(value: &T) -> Result<Self, ReadModelWindowCursorError>
    where
        T: Serialize + ?Sized,
    {
        let json = serde_json::to_vec(value)?;
        Ok(Self(URL_SAFE_NO_PAD.encode(json)))
    }

    pub fn try_to_deserialized<T>(&self) -> Result<T, ReadModelWindowCursorError>
    where
        T: DeserializeOwned,
    {
        let json = URL_SAFE_NO_PAD.decode(&self.0)?;
        Ok(serde_json::from_slice(&json)?)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<String> for ReadModelWindowCursor {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ReadModelWindowCursor {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl AsRef<str> for ReadModelWindowCursor {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for ReadModelWindowCursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Position {
        name: String,
        id: u64,
    }

    #[test]
    fn round_trips_typed_positions_as_url_safe_unpadded_json() {
        let position = Position {
            name: "東京 / + =".to_owned(),
            id: 42,
        };
        let cursor = ReadModelWindowCursor::try_from_serializable(&position).unwrap();
        assert!(
            cursor
                .value()
                .bytes()
                .all(|byte| { byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' })
        );
        let json = URL_SAFE_NO_PAD.decode(cursor.value()).unwrap();
        assert_eq!(json, serde_json::to_vec(&position).unwrap());
        let serialized_cursor = serde_json::to_string(&cursor).unwrap();
        let restored: ReadModelWindowCursor = serde_json::from_str(&serialized_cursor).unwrap();
        assert_eq!(
            restored.try_to_deserialized::<Position>().unwrap(),
            position
        );
    }

    #[test]
    fn reports_invalid_encoding_json_and_target_types() {
        assert!(matches!(
            ReadModelWindowCursor::from("!").try_to_deserialized::<Position>(),
            Err(ReadModelWindowCursorError::Base64(_))
        ));
        let invalid_json = ReadModelWindowCursor::new(URL_SAFE_NO_PAD.encode(b"not json"));
        assert!(matches!(
            invalid_json.try_to_deserialized::<Position>(),
            Err(ReadModelWindowCursorError::Json(_))
        ));
        let incompatible = ReadModelWindowCursor::try_from_serializable("not a position").unwrap();
        assert!(matches!(
            incompatible.try_to_deserialized::<Position>(),
            Err(ReadModelWindowCursorError::Json(_))
        ));
    }

    #[test]
    fn propagates_serialization_errors() {
        let value = BTreeMap::from([(vec![1, 2], 3)]);
        assert!(matches!(
            ReadModelWindowCursor::try_from_serializable(&value),
            Err(ReadModelWindowCursorError::Json(_))
        ));
    }
}
