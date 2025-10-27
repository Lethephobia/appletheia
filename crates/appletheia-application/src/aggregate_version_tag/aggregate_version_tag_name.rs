use std::fmt::{self, Display};

use super::AggregateVersionTagNameError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct AggregateVersionTagName(String);

impl AggregateVersionTagName {
    const MAX_LENGTH: usize = 100;

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for AggregateVersionTagName {
    type Error = AggregateVersionTagNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(AggregateVersionTagNameError::Empty);
        }
        if value.len() > Self::MAX_LENGTH {
            return Err(AggregateVersionTagNameError::TooLong(
                value.len(),
                Self::MAX_LENGTH,
            ));
        }
        Ok(Self(value))
    }
}

impl Display for AggregateVersionTagName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
