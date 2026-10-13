use std::collections::HashSet;

use crate::read_model::ReadModelRef;

use super::ReadModelWatchId;

#[derive(Clone, Debug)]
pub struct ReadModelWatch {
    watch_id: ReadModelWatchId,
    targets: HashSet<ReadModelRef>,
}

impl ReadModelWatch {
    pub fn new() -> Self {
        Self {
            watch_id: ReadModelWatchId::new(),
            targets: HashSet::new(),
        }
    }

    pub fn watch_id(&self) -> ReadModelWatchId {
        self.watch_id
    }

    pub fn targets(&self) -> &HashSet<ReadModelRef> {
        &self.targets
    }

    pub fn add_target(&mut self, target: ReadModelRef) {
        self.targets.insert(target);
    }

    pub fn remove_target(&mut self, target: &ReadModelRef) {
        self.targets.remove(target);
    }
}

impl Default for ReadModelWatch {
    fn default() -> Self {
        Self::new()
    }
}
