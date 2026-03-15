use super::UniqueEntries;

/// Describes the namespace-scoped unique keys derived from aggregate state.
pub trait UniqueConstraints<E>: Send + Sync
where
    E: std::error::Error + Send + Sync + 'static,
{
    /// Returns the unique-key definitions derived from the current state.
    fn unique_entries(&self) -> Result<UniqueEntries, E> {
        Ok(UniqueEntries::new())
    }
}

#[cfg(test)]
mod tests {
    use super::UniqueConstraints;
    use crate::aggregate::{UniqueEntries, UniqueKey, UniqueValue, UniqueValuePart, UniqueValues};
    use thiserror::Error;

    #[derive(Debug, Error)]
    #[error("constraint error")]
    struct ConstraintError;

    struct EmptyConstraints;

    impl UniqueConstraints<ConstraintError> for EmptyConstraints {}

    struct WithUniqueKeys;

    impl UniqueConstraints<ConstraintError> for WithUniqueKeys {
        fn unique_entries(&self) -> Result<UniqueEntries, ConstraintError> {
            let value = UniqueValue::new(vec![
                UniqueValuePart::try_from("tenant_123").expect("valid part"),
                UniqueValuePart::try_from("foo@example.com").expect("valid part"),
            ])
            .expect("valid value");
            let mut unique_keys = UniqueEntries::new();
            let values = UniqueValues::new(vec![value]).expect("unique values should be valid");
            let _ = unique_keys.insert(UniqueKey::new("email"), values);

            Ok(unique_keys)
        }
    }

    #[test]
    fn accepts_empty_constraints_by_default() {
        let constraints = EmptyConstraints;
        let unique_keys = constraints
            .unique_entries()
            .expect("empty constraints should succeed");

        assert!(unique_keys.is_empty());
    }

    #[test]
    fn returns_inserted_unique_keys() {
        let constraints = WithUniqueKeys;
        let unique_keys = constraints
            .unique_entries()
            .expect("declared keys should succeed");

        assert_eq!(
            unique_keys
                .get(UniqueKey::new("email"))
                .map(UniqueValues::len),
            Some(1)
        );
    }
}
