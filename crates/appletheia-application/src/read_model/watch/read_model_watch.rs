use std::collections::HashSet;

use super::{ReadModelWatchError, ReadModelWatchId, ReadModelWatchSelector, ReadModelWatchStatus};

/// Owns the selectors for one watch in a local session.
#[derive(Clone, Debug)]
pub struct ReadModelWatch {
    id: ReadModelWatchId,
    selectors: HashSet<ReadModelWatchSelector>,
    status: ReadModelWatchStatus,
}

impl ReadModelWatch {
    pub fn new(
        selectors: impl IntoIterator<Item = ReadModelWatchSelector>,
    ) -> Result<Self, ReadModelWatchError> {
        let unique_selectors: HashSet<_> = selectors.into_iter().collect();
        if unique_selectors.is_empty() {
            return Err(ReadModelWatchError::EmptySelectors);
        }
        Ok(Self {
            id: ReadModelWatchId::new(),
            selectors: unique_selectors,
            status: ReadModelWatchStatus::Registered,
        })
    }

    pub fn id(&self) -> ReadModelWatchId {
        self.id
    }

    pub fn selectors(&self) -> &HashSet<ReadModelWatchSelector> {
        &self.selectors
    }

    pub fn status(&self) -> ReadModelWatchStatus {
        self.status
    }

    pub(super) fn complete_registration(&mut self) {
        self.status = ReadModelWatchStatus::Registered;
    }

    pub(super) fn begin_removal(&mut self) {
        self.status = ReadModelWatchStatus::Removing;
    }
}
