use std::collections::{HashMap, HashSet};

use crate::read_model::ReadModelRef;

use super::{ReadModelWatch, ReadModelWatchId};

#[derive(Debug, Default)]
pub(super) struct InMemoryReadModelWatchRegistryState {
    pub(super) watches: HashMap<ReadModelWatchId, ReadModelWatch>,
    pub(super) watches_by_target: HashMap<ReadModelRef, HashSet<ReadModelWatchId>>,
}
