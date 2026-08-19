use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

use crate::read_model::ReadModelDependency;

use super::{
    ReadModelWatchRefreshRequest, ReadModelWatchRefreshValue, ReadModelWatchRevision,
    ReadModelWatchSessionId, ReadModelWatchSubscriptionExecutor, ReadModelWatchSubscriptionId,
};

pub(super) type ReadModelWatchSubscriptionAddress =
    (ReadModelWatchSessionId, ReadModelWatchSubscriptionId);

pub(super) struct ReadModelWatchRegistryState {
    pub(super) sessions: HashMap<ReadModelWatchSessionId, Instant>,
    pub(super) subscriptions:
        HashMap<ReadModelWatchSubscriptionAddress, ReadModelWatchSubscriptionState>,
    pub(super) subscriptions_by_dependency:
        HashMap<ReadModelDependency, HashSet<ReadModelWatchSubscriptionAddress>>,
}

impl ReadModelWatchRegistryState {
    pub(super) fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            subscriptions: HashMap::new(),
            subscriptions_by_dependency: HashMap::new(),
        }
    }

    pub(super) fn index(
        &mut self,
        address: ReadModelWatchSubscriptionAddress,
        dependencies: impl IntoIterator<Item = ReadModelDependency>,
    ) {
        for dependency in dependencies {
            self.subscriptions_by_dependency
                .entry(dependency)
                .or_default()
                .insert(address);
        }
    }

    pub(super) fn unindex(
        &mut self,
        address: &ReadModelWatchSubscriptionAddress,
        dependencies: impl IntoIterator<Item = ReadModelDependency>,
    ) {
        for dependency in dependencies {
            let remove_dependency = self
                .subscriptions_by_dependency
                .get_mut(&dependency)
                .map(|addresses| {
                    addresses.remove(address);
                    addresses.is_empty()
                })
                .unwrap_or(false);
            if remove_dependency {
                self.subscriptions_by_dependency.remove(&dependency);
            }
        }
    }
}

pub(super) struct ReadModelWatchSubscriptionState {
    pub(super) executor: Arc<dyn ReadModelWatchSubscriptionExecutor>,
    pub(super) refresh_request: ReadModelWatchRefreshRequest,
    pub(super) prospective_dependencies: HashSet<ReadModelDependency>,
    pub(super) materialized_dependencies: HashSet<ReadModelDependency>,
    pub(super) revision: ReadModelWatchRevision,
    pub(super) last_value: Option<ReadModelWatchRefreshValue>,
    pub(super) refreshing: bool,
    pub(super) dirty: bool,
}
