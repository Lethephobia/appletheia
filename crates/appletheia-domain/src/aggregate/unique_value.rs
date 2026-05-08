use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::{UniqueValueError, UniqueValuePart};

/// Represents the normalized value of a unique key, independent of its namespace.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct UniqueValue {
    parts: Vec<UniqueValuePart>,
}

impl UniqueValue {
    /// Creates a value from one or more normalized parts.
    pub fn new(parts: Vec<UniqueValuePart>) -> Result<Self, UniqueValueError> {
        if parts.is_empty() {
            return Err(UniqueValueError::EmptyParts);
        }

        Ok(Self { parts })
    }

    /// Creates a value from one or more normalized string parts.
    pub fn from_strings<I, S>(parts: I) -> Result<Self, UniqueValueError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let parts = parts
            .into_iter()
            .map(|part| UniqueValuePart::try_from(part.into()))
            .collect::<Result<Vec<_>, _>>()?;

        Self::new(parts)
    }

    /// Returns the normalized parts of this value.
    pub fn parts(&self) -> &[UniqueValuePart] {
        &self.parts
    }

    /// Returns a canonical string representation suitable for persistence.
    pub fn normalized_key(&self) -> String {
        self.parts
            .iter()
            .map(|part| {
                let value = part.value();
                format!("{}:{value}", value.len())
            })
            .collect::<Vec<_>>()
            .join("|")
    }
}

impl Display for UniqueValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.normalized_key())
    }
}

#[cfg(test)]
mod tests {
    use super::UniqueValue;
    use crate::aggregate::{UniqueValueError, UniqueValuePart};

    #[test]
    fn rejects_empty_parts() {
        let error = UniqueValue::new(Vec::new()).expect_err("empty parts should fail");

        assert!(matches!(error, UniqueValueError::EmptyParts));
    }

    #[test]
    fn builds_from_string_parts() {
        let value = UniqueValue::from_strings(["tenant_123", "foo@example.com"])
            .expect("valid string parts should build value");

        assert_eq!(value.normalized_key(), "10:tenant_123|15:foo@example.com");
    }

    #[test]
    fn rejects_empty_string_part() {
        let error = UniqueValue::from_strings(["tenant_123", ""])
            .expect_err("empty string part should fail");

        assert!(matches!(error, UniqueValueError::Part(_)));
    }

    #[test]
    fn builds_canonical_normalized_key() {
        let tenant = UniqueValuePart::try_from("tenant_123").expect("valid part");
        let phone = UniqueValuePart::try_from("+81-90-1234-5678").expect("valid part");
        let value = UniqueValue::new(vec![tenant, phone]).expect("valid value");

        assert_eq!(value.normalized_key(), "10:tenant_123|16:+81-90-1234-5678");
    }
}
