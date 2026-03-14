use std::collections::BTreeMap;

use super::{UniqueKey, UniqueValues};

/// Stores namespace-scoped unique-key values derived from aggregate state.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UniqueEntries(BTreeMap<UniqueKey, UniqueValues>);

impl UniqueEntries {
    /// Creates an empty set of unique-key values.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Replaces all values for the given unique key and returns the previous values.
    pub fn insert(&mut self, unique_key: UniqueKey, values: UniqueValues) -> Option<UniqueValues> {
        self.0.insert(unique_key, values)
    }

    /// Returns the values stored for the given unique key, if present.
    pub fn get(&self, unique_key: UniqueKey) -> Option<&UniqueValues> {
        self.0.get(&unique_key)
    }

    /// Returns an iterator over all unique-key definitions.
    pub fn iter(&self) -> impl Iterator<Item = (&UniqueKey, &UniqueValues)> {
        self.0.iter()
    }

    /// Returns whether there are no unique-key definitions.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl IntoIterator for UniqueEntries {
    type Item = (UniqueKey, UniqueValues);
    type IntoIter = std::collections::btree_map::IntoIter<UniqueKey, UniqueValues>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::UniqueEntries;
    use crate::aggregate::{UniqueKey, UniqueValue, UniqueValuePart, UniqueValues};

    fn values(inputs: &[&str]) -> UniqueValues {
        UniqueValues::new(
            inputs
                .iter()
                .map(|input| {
                    UniqueValue::new(vec![UniqueValuePart::try_from(*input).expect("valid part")])
                        .expect("valid value")
                })
                .collect(),
        )
        .expect("unique values should be valid")
    }

    #[test]
    fn insert_overwrites_previous_values_for_same_key() {
        let mut unique_keys = UniqueEntries::new();
        let first = values(&["a"]);
        let second = values(&["b"]);

        let previous = unique_keys.insert(UniqueKey::new("email"), first.clone());
        assert!(previous.is_none());

        let previous = unique_keys.insert(UniqueKey::new("email"), second.clone());

        assert_eq!(previous, Some(first));
        assert_eq!(unique_keys.get(UniqueKey::new("email")), Some(&second));
    }

    #[test]
    fn iter_returns_unique_keys_in_stable_order() {
        let mut unique_keys = UniqueEntries::new();
        let _ = unique_keys.insert(UniqueKey::new("phone_number"), values(&["09012345678"]));
        let _ = unique_keys.insert(UniqueKey::new("email"), values(&["foo@example.com"]));

        let keys: Vec<_> = unique_keys
            .iter()
            .map(|(unique_key, _)| unique_key.value())
            .collect();

        assert_eq!(keys, vec!["email", "phone_number"]);
    }

    #[test]
    fn insert_accepts_new_unique_key() {
        let mut unique_keys = UniqueEntries::new();
        let value = values(&["a"]);

        let previous = unique_keys.insert(UniqueKey::new("phone_number"), value.clone());

        assert!(previous.is_none());
        assert_eq!(
            unique_keys.get(UniqueKey::new("phone_number")),
            Some(&value)
        );
    }
}
