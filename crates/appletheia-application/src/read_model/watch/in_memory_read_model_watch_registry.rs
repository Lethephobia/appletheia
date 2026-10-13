use tokio::sync::RwLock;

use crate::read_model::ReadModelRef;

use super::in_memory_read_model_watch_registry_state::InMemoryReadModelWatchRegistryState;
use super::{
    ReadModelWatch, ReadModelWatchId, ReadModelWatchRegistry, ReadModelWatchRegistryError,
};

#[derive(Debug, Default)]
pub struct InMemoryReadModelWatchRegistry {
    state: RwLock<InMemoryReadModelWatchRegistryState>,
}

impl InMemoryReadModelWatchRegistry {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ReadModelWatchRegistry for InMemoryReadModelWatchRegistry {
    async fn save(&self, watch: ReadModelWatch) -> Result<(), ReadModelWatchRegistryError> {
        let mut state = self.state.write().await;
        let watch_id = watch.watch_id();
        if let Some(previous) = state.watches.remove(&watch_id) {
            for target in previous.targets().difference(watch.targets()) {
                if let Some(watches) = state.watches_by_target.get_mut(target) {
                    watches.remove(&watch_id);
                    if watches.is_empty() {
                        state.watches_by_target.remove(target);
                    }
                }
            }
        }
        for target in watch.targets() {
            state
                .watches_by_target
                .entry(target.clone())
                .or_default()
                .insert(watch_id);
        }
        state.watches.insert(watch_id, watch);
        Ok(())
    }

    async fn find_by_id(
        &self,
        watch_id: ReadModelWatchId,
    ) -> Result<Option<ReadModelWatch>, ReadModelWatchRegistryError> {
        Ok(self.state.read().await.watches.get(&watch_id).cloned())
    }

    async fn find_by_target(
        &self,
        target: &ReadModelRef,
    ) -> Result<Vec<ReadModelWatchId>, ReadModelWatchRegistryError> {
        let state = self.state.read().await;
        Ok(state
            .watches_by_target
            .get(target)
            .map(|watches| watches.iter().copied().collect())
            .unwrap_or_default())
    }

    async fn remove(&self, watch_id: ReadModelWatchId) -> Result<(), ReadModelWatchRegistryError> {
        let mut state = self.state.write().await;
        if let Some(watch) = state.watches.remove(&watch_id) {
            for target in watch.targets() {
                if let Some(watches) = state.watches_by_target.get_mut(target) {
                    watches.remove(&watch_id);
                    if watches.is_empty() {
                        state.watches_by_target.remove(target);
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::read_model::ReadModelId;

    use super::*;

    #[tokio::test]
    async fn save_updates_targets_without_changing_other_watches() {
        let registry = InMemoryReadModelWatchRegistry::new();
        let first = ReadModelRef {
            read_model_type: "users".parse().unwrap(),
            id: ReadModelId::new("first".to_owned()),
        };
        let second = ReadModelRef {
            read_model_type: "users".parse().unwrap(),
            id: ReadModelId::new("second".to_owned()),
        };
        let mut watch = ReadModelWatch::new();
        let watch_id = watch.watch_id();
        watch.add_target(first.clone());
        watch.add_target(first.clone());
        assert_eq!(watch.targets().len(), 1);
        registry.save(watch).await.unwrap();
        let mut other = ReadModelWatch::new();
        let other_id = other.watch_id();
        other.add_target(first.clone());
        registry.save(other).await.unwrap();
        assert_eq!(
            registry
                .find_by_target(&first)
                .await
                .unwrap()
                .into_iter()
                .collect::<HashSet<_>>(),
            HashSet::from([watch_id, other_id]),
        );

        let mut watch = registry.find_by_id(watch_id).await.unwrap().unwrap();
        watch.remove_target(&first);
        watch.remove_target(&first);
        watch.add_target(second.clone());
        assert!(
            registry
                .find_by_id(watch_id)
                .await
                .unwrap()
                .unwrap()
                .targets()
                .contains(&first)
        );
        assert!(registry.find_by_target(&second).await.unwrap().is_empty());
        registry.save(watch.clone()).await.unwrap();
        registry.save(watch).await.unwrap();
        assert_eq!(
            registry.find_by_target(&first).await.unwrap(),
            vec![other_id]
        );
        assert_eq!(
            registry.find_by_target(&second).await.unwrap(),
            vec![watch_id]
        );

        registry.remove(other_id).await.unwrap();
        registry.remove(other_id).await.unwrap();
        assert!(registry.find_by_id(other_id).await.unwrap().is_none());
        assert!(registry.find_by_target(&first).await.unwrap().is_empty());
        assert_eq!(
            registry.find_by_target(&second).await.unwrap(),
            vec![watch_id]
        );
        registry.remove(watch_id).await.unwrap();
        let state = registry.state.read().await;
        assert!(state.watches.is_empty());
        assert!(state.watches_by_target.is_empty());
    }

    #[tokio::test]
    async fn saving_an_empty_watch_clears_its_reverse_index() {
        let registry = InMemoryReadModelWatchRegistry::new();
        let target = ReadModelRef {
            read_model_type: "users".parse().unwrap(),
            id: ReadModelId::new("first".to_owned()),
        };
        let mut watch = ReadModelWatch::new();
        let watch_id = watch.watch_id();
        assert_eq!(
            watch_id.value().get_version(),
            Some(uuid::Version::SortRand)
        );
        assert!(watch.targets().is_empty());
        registry.save(watch.clone()).await.unwrap();
        assert!(registry.find_by_id(watch_id).await.unwrap().is_some());
        watch.add_target(target.clone());
        registry.save(watch.clone()).await.unwrap();
        watch.remove_target(&target);
        registry.save(watch).await.unwrap();
        assert!(
            registry
                .find_by_id(watch_id)
                .await
                .unwrap()
                .unwrap()
                .targets()
                .is_empty()
        );
        assert!(registry.state.read().await.watches_by_target.is_empty());
    }
}
