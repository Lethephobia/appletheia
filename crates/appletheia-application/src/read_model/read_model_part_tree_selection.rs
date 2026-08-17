use super::{ReadModelPartTree, SerializedPartition, SerializedPartitionError};

/// Contains the fragment partitions and dependencies selected from a materialized part tree.
#[derive(Default)]
pub(super) struct ReadModelPartTreeSelection {
    pub(super) partitions: Vec<SerializedPartition>,
    pub(super) roots: Vec<SerializedPartition>,
    pub(super) dependencies: Vec<(SerializedPartition, Vec<SerializedPartition>)>,
}

impl ReadModelPartTreeSelection {
    pub(super) fn try_from_roots(
        roots: Vec<ReadModelPartTree>,
    ) -> Result<Self, SerializedPartitionError> {
        let mut selection = Self::default();
        for root in roots {
            selection.collect_tree(root, true)?;
        }
        selection.merge_dependencies();

        Ok(selection)
    }

    fn collect_tree(
        &mut self,
        tree: ReadModelPartTree,
        root: bool,
    ) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        let mut tree_partitions = Vec::new();
        for value in tree.values {
            let partition = value.partition?;
            tree_partitions.push(partition.clone());
            if root && !self.roots.contains(&partition) {
                self.roots.push(partition.clone());
            }
            if !self.partitions.contains(&partition) {
                self.partitions.push(partition.clone());
            }

            let mut referenced_partitions = Vec::new();
            let dependency_index = self.dependencies.len();
            for child in value.children {
                let child_partitions = self.collect_tree(child, false)?;
                for child_partition in child_partitions {
                    if child_partition != partition
                        && !referenced_partitions.contains(&child_partition)
                    {
                        referenced_partitions.push(child_partition);
                    }
                }
            }
            if !referenced_partitions.is_empty() {
                self.dependencies
                    .insert(dependency_index, (partition, referenced_partitions));
            }
        }

        Ok(tree_partitions)
    }

    fn merge_dependencies(&mut self) {
        let mut merged = Vec::<(SerializedPartition, Vec<SerializedPartition>)>::new();
        for (partition, references) in self.dependencies.drain(..) {
            if let Some((_, merged_references)) = merged
                .iter_mut()
                .find(|(merged_partition, _)| merged_partition == &partition)
            {
                for reference in references {
                    if !merged_references.contains(&reference) {
                        merged_references.push(reference);
                    }
                }
            } else {
                merged.push((partition, references));
            }
        }
        self.dependencies = merged;
    }
}
