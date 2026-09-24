use std::fmt::{self, Display};

use super::PubsubTopicNameError;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PubsubTopicName(String);

impl PubsubTopicName {
    pub fn new(value: String) -> Result<Self, PubsubTopicNameError> {
        if value.is_empty() {
            return Err(PubsubTopicNameError::Empty);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PubsubTopicName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl From<PubsubTopicName> for String {
    fn from(value: PubsubTopicName) -> Self {
        value.0
    }
}

impl Display for PubsubTopicName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty() {
        let err =
            PubsubTopicName::new(String::new()).expect_err("empty topic name should be rejected");
        assert!(matches!(err, PubsubTopicNameError::Empty));
    }
}
