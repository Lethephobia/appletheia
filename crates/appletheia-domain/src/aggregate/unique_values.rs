use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{UniqueValue, UniqueValuesError};

/// Stores one or more distinct values for a single `UniqueKey` namespace.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UniqueValues(Vec<UniqueValue>);

impl UniqueValues {
    /// Creates a non-empty collection of distinct unique values.
    pub fn new(values: Vec<UniqueValue>) -> Result<Self, UniqueValuesError> {
        if values.is_empty() {
            return Err(UniqueValuesError::Empty);
        }

        let mut seen = HashSet::new();
        for value in &values {
            if !seen.insert(value.clone()) {
                return Err(UniqueValuesError::DuplicateValue {
                    value: value.clone(),
                });
            }
        }

        Ok(Self(values))
    }

    /// Returns the contained values as a slice.
    pub fn as_slice(&self) -> &[UniqueValue] {
        &self.0
    }

    /// Returns the number of contained values.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the first contained value.
    pub fn first(&self) -> Option<&UniqueValue> {
        self.0.first()
    }

    /// Returns an iterator over the contained values.
    pub fn iter(&self) -> impl Iterator<Item = &UniqueValue> {
        self.0.iter()
    }
}

impl IntoIterator for UniqueValues {
    type Item = UniqueValue;
    type IntoIter = std::vec::IntoIter<UniqueValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::UniqueValues;
    use crate::aggregate::{UniqueValue, UniqueValuePart, UniqueValuesError};

    fn value(input: &str) -> UniqueValue {
        UniqueValue::new(vec![UniqueValuePart::try_from(input).expect("valid part")])
            .expect("valid unique value")
    }

    #[test]
    fn rejects_empty_values() {
        let error = UniqueValues::new(Vec::new()).expect_err("empty values should fail");

        assert_eq!(error, UniqueValuesError::Empty);
    }

    #[test]
    fn rejects_duplicate_values() {
        let email = value("foo@example.com");

        let error = UniqueValues::new(vec![email.clone(), email])
            .expect_err("duplicate values should fail");

        assert!(matches!(error, UniqueValuesError::DuplicateValue { .. }));
    }

    #[test]
    fn preserves_values_in_insertion_order() {
        let first = value("a");
        let second = value("b");

        let values = UniqueValues::new(vec![first.clone(), second.clone()])
            .expect("distinct values should succeed");

        assert_eq!(values.as_slice(), vec![first, second].as_slice());
    }
}
