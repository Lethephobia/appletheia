use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;

use crate::read_model::list::{ReadModelListChangeDecision, ReadModelListMatcher};
use crate::read_model::{
    ReadModel, ReadModelFragmentChangeEnvelope, ReadModelNameOwned, ReadModelPartChange,
    ReadModelPartChangeEnvelope, ReadModelPartChangeError, ReadModelPartTreeMapper,
};

use super::{
    DefaultReadModelWatchChangeRouterError, ReadModelTypedListWatch,
    ReadModelWatchPartitionDependencies, ReadModelWatchPartitionState, ReadModelWatchRoute,
};

/// Maps and filters fragment changes for one typed read model watch.
pub struct DefaultReadModelWatchChangeRouter<R>
where
    R: ReadModel,
{
    part_tree_mapper: ReadModelPartTreeMapper,
    read_model: PhantomData<fn() -> R>,
}

impl<R> DefaultReadModelWatchChangeRouter<R>
where
    R: ReadModel,
{
    pub fn new() -> Self {
        Self {
            part_tree_mapper: ReadModelPartTreeMapper::for_read_model::<R>(),
            read_model: PhantomData,
        }
    }

    /// Routes watched sources, matching materialized items, and list invalidations.
    pub fn route(
        &self,
        envelope: &ReadModelFragmentChangeEnvelope,
        partition_state: &ReadModelWatchPartitionState,
    ) -> Result<ReadModelWatchRoute, DefaultReadModelWatchChangeRouterError> {
        self.route_with_evaluator(envelope, partition_state, false, |_, _| {
            Ok(ReadModelListChangeDecision::Ignored)
        })
    }

    /// Routes one typed list watch using its query, coverage, and item matcher.
    pub fn route_list<M>(
        &self,
        envelope: &ReadModelFragmentChangeEnvelope,
        partition_state: &ReadModelWatchPartitionState,
        matcher: &M,
        watched_list: &ReadModelTypedListWatch<M::Query, M::Cursor>,
    ) -> Result<ReadModelWatchRoute, DefaultReadModelWatchChangeRouterError>
    where
        M: ReadModelListMatcher,
    {
        self.route_with_evaluator(
            envelope,
            partition_state,
            true,
            |change, source_is_watched| {
                matcher.evaluate(
                    &watched_list.query,
                    &watched_list.coverage,
                    change,
                    source_is_watched,
                )
            },
        )
    }

    fn route_with_evaluator<F>(
        &self,
        envelope: &ReadModelFragmentChangeEnvelope,
        partition_state: &ReadModelWatchPartitionState,
        watches_list: bool,
        mut evaluate: F,
    ) -> Result<ReadModelWatchRoute, DefaultReadModelWatchChangeRouterError>
    where
        F: FnMut(
            &ReadModelPartChange,
            bool,
        ) -> Result<ReadModelListChangeDecision, ReadModelPartChangeError>,
    {
        let typed_changes = self.part_tree_mapper.map(envelope)?;
        let referenced_partitions = typed_changes
            .iter()
            .map(|change| change.referenced_partitions().to_vec())
            .collect::<Vec<_>>();
        let mut references_by_partition = HashMap::<_, Vec<_>>::new();
        for (change, references) in typed_changes.iter().zip(referenced_partitions.iter()) {
            let source_references = references_by_partition
                .entry(change.source_partition().clone())
                .or_default();
            for reference in references {
                if !source_references.contains(reference) {
                    source_references.push(reference.clone());
                }
            }
        }

        let mut list_decisions = Vec::with_capacity(typed_changes.len());
        for change in &typed_changes {
            let source_is_watched = partition_state.contains(change.source_partition());
            let decision = evaluate(change, source_is_watched)?;
            list_decisions.push(decision);
        }

        let mut admitted_partitions = HashSet::new();
        if watches_list {
            for (change, decision) in typed_changes.iter().zip(list_decisions.iter()) {
                if !partition_state.contains(change.source_partition())
                    && !change.removes_partition()
                    && matches!(decision, ReadModelListChangeDecision::Included)
                {
                    admitted_partitions.insert(change.source_partition().clone());
                }
            }
        } else {
            loop {
                let previous_count = admitted_partitions.len();
                for change in &typed_changes {
                    if partition_state.contains(change.source_partition())
                        || admitted_partitions.contains(change.source_partition())
                        || change.removes_partition()
                    {
                        continue;
                    }
                    if change.audience_partitions().iter().any(|partition| {
                        partition_state.contains(partition)
                            || admitted_partitions.contains(partition)
                    }) {
                        admitted_partitions.insert(change.source_partition().clone());
                    }
                }
                if admitted_partitions.len() == previous_count {
                    break;
                }
            }
        }

        let mut routed_partitions = HashSet::new();
        let mut partitions_to_add = Vec::new();
        let mut partitions_to_add_set = HashSet::new();
        let mut partitions_to_remove = Vec::new();
        let mut partitions_to_remove_set = HashSet::new();
        for change in &typed_changes {
            let source_partition = change.source_partition();
            let source_is_visible = partition_state.contains(source_partition)
                || admitted_partitions.contains(source_partition);
            let removed_source_is_visible_through_audience = !watches_list
                && change.removes_partition()
                && change.audience_partitions().iter().any(|partition| {
                    partition_state.contains(partition) || admitted_partitions.contains(partition)
                });
            if source_is_visible || removed_source_is_visible_through_audience {
                routed_partitions.insert(source_partition.clone());
            }
            if admitted_partitions.contains(source_partition)
                && partitions_to_add_set.insert(source_partition.clone())
            {
                partitions_to_add.push(source_partition.clone());
            }
            if (source_is_visible || removed_source_is_visible_through_audience)
                && change.removes_partition()
                && partitions_to_remove_set.insert(source_partition.clone())
            {
                partitions_to_remove.push(source_partition.clone());
            }
        }

        let mut pending_partitions = routed_partitions.iter().cloned().collect::<VecDeque<_>>();
        while let Some(partition) = pending_partitions.pop_front() {
            let Some(references) = references_by_partition.get(&partition) else {
                continue;
            };
            for referenced_partition in references {
                if routed_partitions.insert(referenced_partition.clone()) {
                    pending_partitions.push_back(referenced_partition.clone());
                }
            }
        }

        let routed_changes = typed_changes
            .iter()
            .filter(|change| routed_partitions.contains(change.source_partition()))
            .cloned()
            .collect::<Vec<_>>();
        let mut dependency_replacements = Vec::new();
        let mut replaced_partitions = HashSet::new();
        for change in &typed_changes {
            let source_partition = change.source_partition();
            if routed_partitions.contains(source_partition)
                && replaced_partitions.insert(source_partition.clone())
            {
                dependency_replacements.push(ReadModelWatchPartitionDependencies {
                    partition: source_partition.clone(),
                    referenced_partitions: references_by_partition
                        .get(source_partition)
                        .cloned()
                        .unwrap_or_default(),
                });
            }
        }

        let change = if routed_changes.is_empty() {
            None
        } else {
            Some(ReadModelPartChangeEnvelope::try_from_fragment_envelope(
                envelope,
                ReadModelNameOwned::from(R::NAME),
                routed_changes,
            )?)
        };

        Ok(ReadModelWatchRoute {
            change,
            list_invalidated: list_decisions
                .iter()
                .any(|decision| matches!(decision, ReadModelListChangeDecision::Invalidated)),
            partitions_to_add,
            partitions_to_remove,
            dependency_replacements,
        })
    }
}

impl<R> Default for DefaultReadModelWatchChangeRouter<R>
where
    R: ReadModel,
{
    fn default() -> Self {
        Self::new()
    }
}
