use std::fmt::{self, Display};

use super::TopicIdError;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TopicId(String);

impl TopicId {
    pub fn new(value: String) -> Result<Self, TopicIdError> {
        if value.is_empty() {
            return Err(TopicIdError::Empty);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TopicId {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl From<TopicId> for String {
    fn from(value: TopicId) -> Self {
        value.0
    }
}

impl Display for TopicId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty() {
        let err = TopicId::new(String::new()).expect_err("empty topic id should be rejected");
        assert!(matches!(err, TopicIdError::Empty));
    }
}
