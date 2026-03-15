use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::UniqueValuePartError;

/// Represents a normalized component of a unique key.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UniqueValuePart(String);

impl UniqueValuePart {
    /// Creates a unique-key part from an already-normalized value.
    pub fn new(value: String) -> Result<Self, UniqueValuePartError> {
        if value.is_empty() {
            return Err(UniqueValuePartError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the underlying normalized value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for UniqueValuePart {
    type Error = UniqueValuePartError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for UniqueValuePart {
    type Error = UniqueValuePartError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Display for UniqueValuePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::{UniqueValuePart, UniqueValuePartError};

    #[test]
    fn accepts_non_empty_value() {
        let part = UniqueValuePart::try_from("tenant_123").expect("part should be valid");

        assert_eq!(part.value(), "tenant_123");
    }

    #[test]
    fn rejects_empty_value() {
        let error = UniqueValuePart::try_from("").expect_err("empty part should fail");

        assert!(matches!(error, UniqueValuePartError::Empty));
    }
}
