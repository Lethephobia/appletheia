use std::collections::{HashMap, HashSet};

use crate::read_model::SerializedPartition;

use super::{ReadModelWatchSession, ReadModelWatchSessionId};

#[derive(Default)]
pub(crate) struct ReadModelWatchSessionRegistryState {
    pub(super) sessions: HashMap<ReadModelWatchSessionId, ReadModelWatchSession>,
    pub(super) session_ids_by_partition:
        HashMap<SerializedPartition, HashSet<ReadModelWatchSessionId>>,
}

impl ReadModelWatchSessionRegistryState {
    pub(super) fn replace_partition_index(
        &mut self,
        session_id: ReadModelWatchSessionId,
        old_partitions: HashSet<SerializedPartition>,
        new_partitions: HashSet<SerializedPartition>,
    ) {
        for partition in old_partitions.difference(&new_partitions) {
            if let Some(session_ids) = self.session_ids_by_partition.get_mut(partition) {
                session_ids.remove(&session_id);
                if session_ids.is_empty() {
                    self.session_ids_by_partition.remove(partition);
                }
            }
        }
        for partition in new_partitions {
            self.session_ids_by_partition
                .entry(partition)
                .or_default()
                .insert(session_id);
        }
    }
}
