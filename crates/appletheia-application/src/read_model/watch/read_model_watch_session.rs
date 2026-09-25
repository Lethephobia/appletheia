use std::collections::HashMap;

use super::{
    ReadModelWatch, ReadModelWatchId, ReadModelWatchSessionError, ReadModelWatchSessionId,
    ReadModelWatchSessionStatus, ReadModelWatchStatus,
};

/// Owns local watches associated with a client session.
#[derive(Debug)]
pub struct ReadModelWatchSession {
    id: ReadModelWatchSessionId,
    watches: HashMap<ReadModelWatchId, ReadModelWatch>,
    status: ReadModelWatchSessionStatus,
}

impl ReadModelWatchSession {
    pub fn new() -> Self {
        Self {
            id: ReadModelWatchSessionId::new(),
            watches: HashMap::new(),
            status: ReadModelWatchSessionStatus::Open,
        }
    }

    pub fn id(&self) -> ReadModelWatchSessionId {
        self.id
    }

    pub fn watches(&self) -> &HashMap<ReadModelWatchId, ReadModelWatch> {
        &self.watches
    }

    pub fn status(&self) -> ReadModelWatchSessionStatus {
        self.status
    }

    pub fn register_watch(
        &mut self,
        mut watch: ReadModelWatch,
    ) -> Result<(), ReadModelWatchSessionError> {
        self.ensure_watch_registration_allowed(watch.id())?;
        watch.complete_registration();
        self.watches.insert(watch.id(), watch);
        Ok(())
    }

    pub fn ensure_watch_registration_allowed(
        &self,
        watch_id: ReadModelWatchId,
    ) -> Result<(), ReadModelWatchSessionError> {
        if self.status == ReadModelWatchSessionStatus::Closing {
            return Err(ReadModelWatchSessionError::Closing);
        }
        if self
            .watches
            .get(&watch_id)
            .is_some_and(|existing| existing.status() == ReadModelWatchStatus::Removing)
        {
            return Err(ReadModelWatchSessionError::WatchRemoving(watch_id));
        }
        Ok(())
    }

    pub fn begin_watch_removal(&mut self, watch_id: ReadModelWatchId) {
        if let Some(watch) = self.watches.get_mut(&watch_id) {
            watch.begin_removal();
        }
    }

    pub fn begin_close(&mut self) {
        self.status = ReadModelWatchSessionStatus::Closing;
        for watch in self.watches.values_mut() {
            watch.begin_removal();
        }
    }

    pub fn complete_index_synchronization(&mut self) {
        self.watches
            .retain(|_, watch| watch.status() != ReadModelWatchStatus::Removing);
    }
}

impl Default for ReadModelWatchSession {
    fn default() -> Self {
        Self::new()
    }
}
