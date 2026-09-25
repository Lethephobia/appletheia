use super::ReadModelPartition;

/// Collects the physical partitions invalidated by one projector execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelInvalidatedPartitions<K> {
    partitions: Vec<ReadModelPartition<K>>,
}

impl<K> ReadModelInvalidatedPartitions<K> {
    pub fn new() -> Self {
        Self {
            partitions: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.partitions.is_empty()
    }

    pub fn len(&self) -> usize {
        self.partitions.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ReadModelPartition<K>> {
        self.partitions.iter()
    }

    pub fn insert(&mut self, key: K) -> bool
    where
        K: PartialEq,
    {
        let partition = ReadModelPartition::new(key);
        if self.partitions.contains(&partition) {
            return false;
        }
        self.partitions.push(partition);
        true
    }
}

impl<K> Default for ReadModelInvalidatedPartitions<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K> IntoIterator for ReadModelInvalidatedPartitions<K> {
    type Item = ReadModelPartition<K>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.partitions.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserting_keys_deduplicates_partitions_and_preserves_order() {
        let mut partitions = ReadModelInvalidatedPartitions::new();
        assert!(partitions.is_empty());
        assert!(partitions.insert(2));
        assert!(partitions.insert(1));
        assert!(!partitions.insert(2));
        assert_eq!(partitions.len(), 2);
        assert_eq!(
            partitions
                .iter()
                .map(|partition| *partition.key())
                .collect::<Vec<_>>(),
            vec![2, 1]
        );
    }
}
