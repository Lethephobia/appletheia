use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::ObjectBucketNameError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectBucketName(String);

impl ObjectBucketName {
    pub fn new(value: String) -> Result<Self, ObjectBucketNameError> {
        if value.is_empty() {
            return Err(ObjectBucketNameError::Empty);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ObjectBucketName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for ObjectBucketName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<ObjectBucketName> for String {
    fn from(value: ObjectBucketName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_bucket_name() {
        let error =
            ObjectBucketName::new(String::new()).expect_err("empty bucket name should be rejected");

        assert!(matches!(error, ObjectBucketNameError::Empty));
    }
}
