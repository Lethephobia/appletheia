use std::fmt::{self, Display};

use crate::saga::SagaName;

use super::ConsumerGroupError;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ConsumerGroup(String);

impl ConsumerGroup {
    pub fn new(value: String) -> Result<Self, ConsumerGroupError> {
        if value.is_empty() {
            return Err(ConsumerGroupError::Empty);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<SagaName> for ConsumerGroup {
    fn from(value: SagaName) -> Self {
        Self(value.value().to_string())
    }
}

impl AsRef<str> for ConsumerGroup {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl From<ConsumerGroup> for String {
    fn from(value: ConsumerGroup) -> Self {
        value.0
    }
}

impl Display for ConsumerGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
