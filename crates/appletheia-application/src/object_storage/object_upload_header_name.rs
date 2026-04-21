use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::ObjectUploadHeaderNameError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectUploadHeaderName(String);

impl ObjectUploadHeaderName {
    pub fn new(value: String) -> Result<Self, ObjectUploadHeaderNameError> {
        if value.is_empty() {
            return Err(ObjectUploadHeaderNameError::Empty);
        }
        if !value
            .as_bytes()
            .iter()
            .all(|&b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return Err(ObjectUploadHeaderNameError::InvalidFormat);
        }

        Ok(Self(value.to_ascii_lowercase()))
    }

    pub fn content_length() -> Self {
        Self("content-length".to_owned())
    }

    pub fn content_md5() -> Self {
        Self("content-md5".to_owned())
    }

    pub fn content_type() -> Self {
        Self("content-type".to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ObjectUploadHeaderName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for ObjectUploadHeaderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<ObjectUploadHeaderName> for String {
    fn from(value: ObjectUploadHeaderName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_normalizes_to_lowercase() {
        let name = ObjectUploadHeaderName::new("Content-Type".to_owned())
            .expect("header name should be valid");

        assert_eq!(name.as_str(), "content-type");
    }

    #[test]
    fn new_rejects_invalid_format() {
        let error = ObjectUploadHeaderName::new("content type".to_owned())
            .expect_err("spaces should be rejected");

        assert!(matches!(error, ObjectUploadHeaderNameError::InvalidFormat));
    }
}
