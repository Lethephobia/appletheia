use std::collections::BTreeMap;

use super::{ReferenceKey, ReferenceValues};

/// Stores reference-index values derived from aggregate state.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReferenceEntries(BTreeMap<ReferenceKey, ReferenceValues>);

impl ReferenceEntries {
    /// Creates an empty set of reference-index values.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Replaces all values for the given reference key and returns the previous values.
    pub fn insert(
        &mut self,
        reference_key: ReferenceKey,
        values: ReferenceValues,
    ) -> Option<ReferenceValues> {
        self.0.insert(reference_key, values)
    }

    /// Returns the values stored for the given reference key, if present.
    pub fn get(&self, reference_key: ReferenceKey) -> Option<&ReferenceValues> {
        self.0.get(&reference_key)
    }

    /// Returns an iterator over all reference-index definitions.
    pub fn iter(&self) -> impl Iterator<Item = (&ReferenceKey, &ReferenceValues)> {
        self.0.iter()
    }

    /// Returns whether there are no reference-index definitions.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl IntoIterator for ReferenceEntries {
    type Item = (ReferenceKey, ReferenceValues);
    type IntoIter = std::collections::btree_map::IntoIter<ReferenceKey, ReferenceValues>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
