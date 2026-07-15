use std::collections::HashSet;

use uuid::Uuid;

use super::{AggregateId, ReferenceValuesError};

/// Stores target aggregate IDs for one reference key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceValues(Vec<Uuid>);

impl ReferenceValues {
    /// Creates reference values from target aggregate IDs.
    pub fn new<I>(ids: Vec<I>) -> Result<Self, ReferenceValuesError>
    where
        I: AggregateId,
    {
        if ids.is_empty() {
            return Err(ReferenceValuesError::Empty);
        }

        let values = ids.into_iter().map(|id| id.value()).collect::<Vec<_>>();
        let mut seen = HashSet::new();
        for value in &values {
            if !seen.insert(*value) {
                return Err(ReferenceValuesError::DuplicateValue { value: *value });
            }
        }

        Ok(Self(values))
    }

    /// Returns the number of referenced target IDs.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether there are no referenced target IDs.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over the persisted target aggregate IDs.
    pub fn iter(&self) -> impl Iterator<Item = Uuid> + '_ {
        self.0.iter().copied()
    }
}

impl IntoIterator for ReferenceValues {
    type Item = Uuid;
    type IntoIter = std::vec::IntoIter<Uuid>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use uuid::Uuid;

    use super::ReferenceValues;
    use crate::aggregate::{AggregateId, ReferenceValuesError};

    #[derive(Debug, Error, Eq, PartialEq)]
    #[allow(dead_code)]
    enum TestIdError {
        #[error("invalid id")]
        Invalid,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct TestId(Uuid);

    impl AggregateId for TestId {
        type Error = TestIdError;

        fn new() -> Self {
            Self(Uuid::now_v7())
        }

        fn value(&self) -> Uuid {
            self.0
        }

        fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
            Ok(Self(value))
        }
    }

    #[test]
    fn rejects_empty_values() {
        let error = ReferenceValues::new::<TestId>(Vec::new()).expect_err("empty should fail");

        assert_eq!(error, ReferenceValuesError::Empty);
    }

    #[test]
    fn rejects_duplicate_values() {
        let id = TestId(Uuid::now_v7());

        let error = ReferenceValues::new(vec![id, id]).expect_err("duplicate values should fail");

        assert!(matches!(error, ReferenceValuesError::DuplicateValue { .. }));
    }

    #[test]
    fn preserves_values_in_insertion_order() {
        let first = TestId(Uuid::now_v7());
        let second = TestId(Uuid::now_v7());

        let values =
            ReferenceValues::new(vec![first, second]).expect("distinct values should succeed");

        assert_eq!(
            values.iter().collect::<Vec<_>>(),
            vec![first.value(), second.value()]
        );
    }
}
