use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::{ObjectChecksum, ObjectContentLength, ObjectContentType, ObjectUploadHeaderValueError};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectUploadHeaderValue(String);

impl ObjectUploadHeaderValue {
    pub fn new(value: String) -> Result<Self, ObjectUploadHeaderValueError> {
        if value.contains(['\r', '\n']) {
            return Err(ObjectUploadHeaderValueError::InvalidFormat);
        }

        Ok(Self(value))
    }

    pub fn from_content_type(value: &ObjectContentType) -> Self {
        Self(value.as_str().to_owned())
    }

    pub fn from_content_length(value: ObjectContentLength) -> Self {
        Self(value.value().to_string())
    }

    pub fn from_checksum(value: &ObjectChecksum) -> Self {
        Self(value.value().as_str().to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ObjectUploadHeaderValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for ObjectUploadHeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<ObjectUploadHeaderValue> for String {
    fn from(value: ObjectUploadHeaderValue) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_line_breaks() {
        let error = ObjectUploadHeaderValue::new("a\r\nb".to_owned())
            .expect_err("line breaks should be rejected");

        assert!(matches!(error, ObjectUploadHeaderValueError::InvalidFormat));
    }
}
