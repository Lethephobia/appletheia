use std::collections::{HashMap, HashSet, VecDeque};

use crate::read_model::SerializedPartition;

use super::{ReadModelWatchPartitionDependencies, ReadModelWatchRoute};

/// Tracks directly watched fragments and the reachable fragments they reference.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReadModelWatchPartitionState {
    direct_partitions: HashSet<SerializedPartition>,
    dependencies: HashMap<SerializedPartition, HashSet<SerializedPartition>>,
    watched_partitions: HashSet<SerializedPartition>,
}

impl ReadModelWatchPartitionState {
    /// Builds a watch state from direct fragments and their complete dependency graph.
    pub fn new(
        direct_partitions: impl IntoIterator<Item = SerializedPartition>,
        dependencies: impl IntoIterator<Item = ReadModelWatchPartitionDependencies>,
    ) -> Self {
        let mut state = Self {
            direct_partitions: direct_partitions.into_iter().collect(),
            ..Self::default()
        };
        state.replace_dependencies_without_recomputing(dependencies);
        state.recompute_watched_partitions();
        state
    }

    /// Returns whether a fragment is reachable from the current direct watch set.
    pub fn contains(&self, partition: &SerializedPartition) -> bool {
        self.watched_partitions.contains(partition)
    }

    /// Returns every directly or transitively watched fragment.
    pub fn watched_partitions(&self) -> &HashSet<SerializedPartition> {
        &self.watched_partitions
    }

    /// Applies direct fragment and dependency changes as one state transition.
    pub fn apply_update(
        &mut self,
        partitions_to_add: &[SerializedPartition],
        partitions_to_remove: &[SerializedPartition],
        dependency_replacements: &[ReadModelWatchPartitionDependencies],
    ) {
        let mut full_recomputation_is_required = false;
        for partition in partitions_to_remove {
            full_recomputation_is_required |= self.direct_partitions.remove(partition);
        }
        let mut pending_partitions = VecDeque::new();
        for partition in partitions_to_add {
            if self.direct_partitions.insert(partition.clone())
                && self.watched_partitions.insert(partition.clone())
            {
                pending_partitions.push_back(partition.clone());
            }
        }
        for replacement in dependency_replacements {
            let referenced_partitions = replacement
                .referenced_partitions
                .iter()
                .cloned()
                .collect::<HashSet<_>>();
            if referenced_partitions.is_empty() {
                full_recomputation_is_required |=
                    self.dependencies.remove(&replacement.partition).is_some();
                continue;
            }

            match self.dependencies.get(&replacement.partition) {
                Some(current_references) if current_references == &referenced_partitions => {}
                Some(_) => {
                    self.dependencies
                        .insert(replacement.partition.clone(), referenced_partitions);
                    full_recomputation_is_required = true;
                }
                None => {
                    self.dependencies
                        .insert(replacement.partition.clone(), referenced_partitions);
                    if self.watched_partitions.contains(&replacement.partition) {
                        pending_partitions.push_back(replacement.partition.clone());
                    }
                }
            }
        }

        if full_recomputation_is_required {
            self.recompute_watched_partitions();
            return;
        }

        self.extend_watched_partitions(pending_partitions);
    }

    /// Installs the subscription-state effects that must precede one routed change.
    pub fn apply_route(&mut self, route: &ReadModelWatchRoute) {
        self.apply_update(
            &route.partitions_to_add,
            &route.partitions_to_remove,
            &route.dependency_replacements,
        );
    }

    fn replace_dependencies_without_recomputing(
        &mut self,
        replacements: impl IntoIterator<Item = ReadModelWatchPartitionDependencies>,
    ) {
        for replacement in replacements {
            let referenced_partitions = replacement
                .referenced_partitions
                .into_iter()
                .collect::<HashSet<_>>();
            if referenced_partitions.is_empty() {
                self.dependencies.remove(&replacement.partition);
            } else {
                self.dependencies
                    .insert(replacement.partition, referenced_partitions);
            }
        }
    }

    fn recompute_watched_partitions(&mut self) {
        let mut watched_partitions = self.direct_partitions.clone();
        let pending_partitions = self
            .direct_partitions
            .iter()
            .cloned()
            .collect::<VecDeque<_>>();
        Self::extend_partition_set(
            &self.dependencies,
            &mut watched_partitions,
            pending_partitions,
        );

        self.dependencies
            .retain(|partition, _| watched_partitions.contains(partition));
        self.watched_partitions = watched_partitions;
    }

    fn extend_watched_partitions(&mut self, pending_partitions: VecDeque<SerializedPartition>) {
        Self::extend_partition_set(
            &self.dependencies,
            &mut self.watched_partitions,
            pending_partitions,
        );
    }

    fn extend_partition_set(
        dependencies: &HashMap<SerializedPartition, HashSet<SerializedPartition>>,
        watched_partitions: &mut HashSet<SerializedPartition>,
        mut pending_partitions: VecDeque<SerializedPartition>,
    ) {
        while let Some(partition) = pending_partitions.pop_front() {
            let Some(referenced_partitions) = dependencies.get(&partition) else {
                continue;
            };
            for referenced_partition in referenced_partitions {
                if watched_partitions.insert(referenced_partition.clone()) {
                    pending_partitions.push_back(referenced_partition.clone());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn partition(name: &str) -> SerializedPartition {
        SerializedPartition::try_from(json!({ "name": name }))
            .expect("test partition should be valid")
    }

    fn dependencies(
        source: &SerializedPartition,
        targets: &[SerializedPartition],
    ) -> ReadModelWatchPartitionDependencies {
        ReadModelWatchPartitionDependencies {
            partition: source.clone(),
            referenced_partitions: targets.to_vec(),
        }
    }

    #[test]
    fn replacing_a_dependency_removes_an_unreferenced_partition() {
        let organization = partition("organization");
        let first_owner = partition("first_owner");
        let second_owner = partition("second_owner");
        let mut state = ReadModelWatchPartitionState::new(
            [organization.clone()],
            [dependencies(
                &organization,
                std::slice::from_ref(&first_owner),
            )],
        );

        state.apply_update(
            &[],
            &[],
            &[dependencies(
                &organization,
                std::slice::from_ref(&second_owner),
            )],
        );

        assert!(state.contains(&organization));
        assert!(!state.contains(&first_owner));
        assert!(state.contains(&second_owner));
    }

    #[test]
    fn replacing_one_shared_dependency_keeps_the_other_reference_alive() {
        let first_organization = partition("first_organization");
        let second_organization = partition("second_organization");
        let shared_owner = partition("shared_owner");
        let replacement_owner = partition("replacement_owner");
        let mut state = ReadModelWatchPartitionState::new(
            [first_organization.clone(), second_organization.clone()],
            [
                dependencies(&first_organization, std::slice::from_ref(&shared_owner)),
                dependencies(&second_organization, std::slice::from_ref(&shared_owner)),
            ],
        );

        state.apply_update(
            &[],
            &[],
            &[dependencies(
                &first_organization,
                std::slice::from_ref(&replacement_owner),
            )],
        );

        assert!(state.contains(&shared_owner));
        assert!(state.contains(&replacement_owner));
    }

    #[test]
    fn removing_the_last_direct_root_removes_its_transitive_dependencies() {
        let organization = partition("organization");
        let owner = partition("owner");
        let picture = partition("picture");
        let mut state = ReadModelWatchPartitionState::new(
            [organization.clone()],
            [
                dependencies(&organization, std::slice::from_ref(&owner)),
                dependencies(&owner, std::slice::from_ref(&picture)),
            ],
        );

        state.apply_update(&[], std::slice::from_ref(&organization), &[]);

        assert!(state.watched_partitions().is_empty());
    }

    #[test]
    fn adding_a_direct_root_installs_its_transitive_dependencies() {
        let organization = partition("organization");
        let owner = partition("owner");
        let picture = partition("picture");
        let mut state = ReadModelWatchPartitionState::default();

        state.apply_update(
            std::slice::from_ref(&organization),
            &[],
            &[
                dependencies(&organization, std::slice::from_ref(&owner)),
                dependencies(&owner, std::slice::from_ref(&picture)),
            ],
        );

        assert!(state.contains(&organization));
        assert!(state.contains(&owner));
        assert!(state.contains(&picture));
    }
}
