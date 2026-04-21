use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::ObjectContentTypeError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
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

    pub fn png() -> Self {
        Self("image/png".to_owned())
    }

    pub fn jpeg() -> Self {
        Self("image/jpeg".to_owned())
    }

    pub fn webp() -> Self {
        Self("image/webp".to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for ObjectContentType {
    type Err = ObjectContentTypeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<&str> for ObjectContentType {
    type Error = ObjectContentTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for ObjectContentType {
    type Error = ObjectContentTypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
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

    #[test]
    fn convenience_constructors_return_common_image_types() {
        assert_eq!(ObjectContentType::png().as_str(), "image/png");
        assert_eq!(ObjectContentType::jpeg().as_str(), "image/jpeg");
        assert_eq!(ObjectContentType::webp().as_str(), "image/webp");
    }

    #[test]
    fn try_from_accepts_valid_content_type() {
        let content_type =
            ObjectContentType::try_from("image/gif").expect("content type should be valid");

        assert_eq!(content_type.as_str(), "image/gif");
    }
}
