use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::ObjectChecksumValueError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectChecksumValue(String);

impl ObjectChecksumValue {
    pub fn new(value: String) -> Result<Self, ObjectChecksumValueError> {
        if value.is_empty() {
            return Err(ObjectChecksumValueError::Empty);
        }
        if value.contains(['\r', '\n']) {
            return Err(ObjectChecksumValueError::InvalidFormat);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ObjectChecksumValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for ObjectChecksumValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<ObjectChecksumValue> for String {
    fn from(value: ObjectChecksumValue) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_checksum_value() {
        let error = ObjectChecksumValue::new(String::new())
            .expect_err("empty checksum value should be rejected");

        assert!(matches!(error, ObjectChecksumValueError::Empty));
    }

    #[test]
    fn new_rejects_line_breaks() {
        let error = ObjectChecksumValue::new("abc\r\nx: y".to_owned())
            .expect_err("line breaks should be rejected");

        assert!(matches!(error, ObjectChecksumValueError::InvalidFormat));
    }
}
