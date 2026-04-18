use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::ObjectContentTypeError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectContentType(String);

impl ObjectContentType {
    pub fn new(value: String) -> Result<Self, ObjectContentTypeError> {
        if value.is_empty() {
            return Err(ObjectContentTypeError::Empty);
        }
        if value.contains(['\r', '\n']) {
            return Err(ObjectContentTypeError::InvalidFormat);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ObjectContentType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for ObjectContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<ObjectContentType> for String {
    fn from(value: ObjectContentType) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_content_type() {
        let error = ObjectContentType::new(String::new())
            .expect_err("empty content type should be rejected");

        assert!(matches!(error, ObjectContentTypeError::Empty));
    }

    #[test]
    fn new_rejects_line_breaks() {
        let error = ObjectContentType::new("text/plain\r\nx: y".to_owned())
            .expect_err("line breaks should be rejected");

        assert!(matches!(error, ObjectContentTypeError::InvalidFormat));
    }
}
