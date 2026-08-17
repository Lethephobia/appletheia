use std::collections::HashSet;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::read_model::SerializedPartition;

use super::read_model_watch_session_registry_state::ReadModelWatchSessionRegistryState;
use super::{ReadModelWatchSession, ReadModelWatchSessionId, ReadModelWatchSessionRegistry};

/// Stores active typed watch sessions and indexes them by physical source partition.
#[derive(Clone)]
pub struct DefaultReadModelWatchSessionRegistry {
    pub(super) state: Arc<RwLock<ReadModelWatchSessionRegistryState>>,
}

impl DefaultReadModelWatchSessionRegistry {
    pub fn new() -> Self {
        Self {
            state: Arc::default(),
        }
    }
}

impl Default for DefaultReadModelWatchSessionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadModelWatchSessionRegistry for DefaultReadModelWatchSessionRegistry {
    async fn register(&self, session_id: ReadModelWatchSessionId, session: ReadModelWatchSession) {
        self.state
            .write()
            .await
            .sessions
            .insert(session_id, session);
    }

    async fn remove(&self, session_id: &ReadModelWatchSessionId) {
        {
            let mut state = self.state.write().await;
            state.sessions.remove(session_id);
        }
    }

    async fn session(&self, session_id: &ReadModelWatchSessionId) -> Option<ReadModelWatchSession> {
        self.state.read().await.sessions.get(session_id).cloned()
    }

    async fn session_ids_for_partition(
        &self,
        partition: &SerializedPartition,
    ) -> Vec<ReadModelWatchSessionId> {
        let mut session_ids = self
            .state
            .read()
            .await
            .session_ids_by_partition
            .get(partition)
            .map(|session_ids| session_ids.iter().copied().collect::<Vec<_>>())
            .unwrap_or_default();
        session_ids.sort();
        session_ids
    }

    async fn replace_partition_index(
        &self,
        session_id: ReadModelWatchSessionId,
        old_partitions: HashSet<SerializedPartition>,
        new_partitions: HashSet<SerializedPartition>,
    ) {
        self.state.write().await.replace_partition_index(
            session_id,
            old_partitions,
            new_partitions,
        );
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::sync::{Arc, Mutex as StdMutex};

    use appletheia_domain::{AggregateVersion, EventId, EventOccurredAt};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
        SerializedEventPayload,
    };
    use crate::projection::ProjectorName;
    use crate::read_model::{
        ReadModel, ReadModelFragment, ReadModelFragmentChange, ReadModelFragmentChangeEnvelope,
        ReadModelFragmentName, ReadModelName, ReadModelObservation, ReadModelObservationSource,
        ReadModelPart, ReadModelPartName, ReadModelPartTree,
    };
    use crate::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };

    use super::*;
    use crate::read_model::watch::{
        DefaultReadModelFragmentChangeDispatcher, DefaultReadModelWatchRegistrar,
        DefaultReadModelWatchSessionOpener, ReadModelFragmentChangeDispatcher,
        ReadModelWatchDelivery, ReadModelWatchDeliveryError, ReadModelWatchRegistrar,
        ReadModelWatchRoute, ReadModelWatchSelection, ReadModelWatchSessionOpener,
        ReadModelWatchSessionRegistry,
    };

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    struct TestFragment {
        id: Uuid,
    }

    impl ReadModelObservationSource for TestFragment {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModelFragment for TestFragment {
        const NAME: ReadModelFragmentName = ReadModelFragmentName::new("watch_test_fragment");
        type Key = Uuid;

        fn key(&self) -> Self::Key {
            self.id
        }
    }

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    struct TestPart {
        id: Uuid,
    }

    impl From<TestFragment> for TestPart {
        fn from(fragment: TestFragment) -> Self {
            Self { id: fragment.id }
        }
    }

    impl ReadModelObservationSource for TestPart {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModelPart for TestPart {
        const NAME: ReadModelPartName = ReadModelPartName::new("watch_test_part");
        type SourceFragment = TestFragment;

        fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
            self.id
        }
    }

    struct TestReadModel {
        item: TestPart,
    }

    impl ReadModelObservationSource for TestReadModel {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModel for TestReadModel {
        const NAME: ReadModelName = ReadModelName::new("watch_test_read_model");

        fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
            vec![ReadModelPartTree::field::<TestPart>(
                "item",
                read_model.map(|read_model| &read_model.item),
            )]
        }
    }

    #[derive(Clone, Default)]
    struct TestDelivery {
        routes: Arc<StdMutex<Vec<ReadModelWatchRoute>>>,
        fail: bool,
    }

    impl ReadModelWatchDelivery for TestDelivery {
        async fn deliver(
            &self,
            _session_id: &ReadModelWatchSessionId,
            route: &ReadModelWatchRoute,
        ) -> Result<(), ReadModelWatchDeliveryError> {
            if self.fail {
                return Err(ReadModelWatchDeliveryError::new(io::Error::other(
                    "delivery failed",
                )));
            }
            self.routes
                .lock()
                .expect("test routes should lock")
                .push(route.clone());
            Ok(())
        }
    }

    fn event_envelope() -> EventEnvelope {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let message_id = MessageId::new();
        let request_context = RequestContext::new(correlation_id, message_id, Principal::System)
            .expect("system request context should be valid");
        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::try_from("watch_test")
                .expect("aggregate type should be valid"),
            aggregate_id: AggregateIdValue::from(Uuid::now_v7()),
            aggregate_version: AggregateVersion::try_from(1).expect("version should be valid"),
            event_name: EventNameOwned::try_from("changed").expect("event name should be valid"),
            payload: SerializedEventPayload::try_from(serde_json::json!({}))
                .expect("payload should be valid"),
            occurred_at: EventOccurredAt::now(),
            correlation_id,
            causation_id: CausationId::from(message_id),
            context: request_context,
        }
    }

    fn envelope(id: Uuid) -> ReadModelFragmentChangeEnvelope {
        let change = ReadModelFragmentChange::try_from_fragment(&TestFragment { id })
            .expect("fragment change should serialize");
        ReadModelFragmentChangeEnvelope::from_changes(
            vec![change],
            &event_envelope(),
            ProjectorName::new("watch_test_projector"),
        )
        .expect("fragment change should finalize")
    }

    fn selection(id: Uuid) -> ReadModelWatchSelection {
        ReadModelWatchSelection::try_from_read_model(&TestReadModel {
            item: TestPart { id },
        })
        .expect("watch selection should serialize")
    }

    async fn open_registered_session(
        registry: &DefaultReadModelWatchSessionRegistry,
        id: Uuid,
        delivery: TestDelivery,
    ) -> ReadModelWatchSessionId {
        let session_opener = DefaultReadModelWatchSessionOpener::new(registry.clone());
        let session_id = session_opener
            .open_snapshot::<TestReadModel, _>(delivery)
            .await;
        DefaultReadModelWatchRegistrar::new(Arc::clone(&registry.state))
            .register(&session_id, selection(id))
            .await
            .expect("selection should register");
        session_id
    }

    #[tokio::test]
    async fn fans_one_shared_partition_out_to_multiple_typed_sessions() {
        let registry = DefaultReadModelWatchSessionRegistry::new();
        let id = Uuid::now_v7();
        let first_delivery = TestDelivery::default();
        let second_delivery = TestDelivery::default();
        open_registered_session(&registry, id, first_delivery.clone()).await;
        open_registered_session(&registry, id, second_delivery.clone()).await;
        let dispatcher = DefaultReadModelFragmentChangeDispatcher::new(registry);

        dispatcher
            .dispatch(&envelope(id))
            .await
            .expect("shared change should fan out");

        assert_eq!(
            first_delivery
                .routes
                .lock()
                .expect("routes should lock")
                .len(),
            1
        );
        assert_eq!(
            second_delivery
                .routes
                .lock()
                .expect("routes should lock")
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn ignores_an_unregistered_partition_and_accepts_no_active_sessions() {
        let registry = DefaultReadModelWatchSessionRegistry::new();
        let watched_id = Uuid::now_v7();
        let delivery = TestDelivery::default();
        open_registered_session(&registry, watched_id, delivery.clone()).await;
        let dispatcher = DefaultReadModelFragmentChangeDispatcher::new(registry);

        dispatcher
            .dispatch(&envelope(Uuid::now_v7()))
            .await
            .expect("an irrelevant partition should be accepted");

        assert!(
            delivery
                .routes
                .lock()
                .expect("routes should lock")
                .is_empty()
        );
    }

    #[tokio::test]
    async fn removing_a_session_removes_it_from_partition_fanout() {
        let registry = DefaultReadModelWatchSessionRegistry::new();
        let id = Uuid::now_v7();
        let delivery = TestDelivery::default();
        let session_id = open_registered_session(&registry, id, delivery.clone()).await;
        registry.remove(&session_id).await;
        let dispatcher = DefaultReadModelFragmentChangeDispatcher::new(registry);

        dispatcher
            .dispatch(&envelope(id))
            .await
            .expect("a closed session should be skipped");

        assert!(
            delivery
                .routes
                .lock()
                .expect("routes should lock")
                .is_empty()
        );
    }

    #[tokio::test]
    async fn replacing_a_selection_deduplicates_and_updates_the_partition_index() {
        let registry = DefaultReadModelWatchSessionRegistry::new();
        let first_id = Uuid::now_v7();
        let second_id = Uuid::now_v7();
        let delivery = TestDelivery::default();
        let session_id = open_registered_session(&registry, first_id, delivery.clone()).await;

        DefaultReadModelWatchRegistrar::new(Arc::clone(&registry.state))
            .register(&session_id, selection(first_id))
            .await
            .expect("duplicate selection should be idempotent");
        DefaultReadModelWatchRegistrar::new(Arc::clone(&registry.state))
            .register(&session_id, selection(second_id))
            .await
            .expect("replacement selection should register");
        let dispatcher = DefaultReadModelFragmentChangeDispatcher::new(registry);

        dispatcher
            .dispatch(&envelope(first_id))
            .await
            .expect("superseded partition should be ignored");
        dispatcher
            .dispatch(&envelope(second_id))
            .await
            .expect("replacement partition should dispatch");

        assert_eq!(delivery.routes.lock().expect("routes should lock").len(), 1);
    }

    #[tokio::test]
    async fn reports_delivery_failure_for_retry() {
        let registry = DefaultReadModelWatchSessionRegistry::new();
        let id = Uuid::now_v7();
        open_registered_session(
            &registry,
            id,
            TestDelivery {
                fail: true,
                ..TestDelivery::default()
            },
        )
        .await;
        let dispatcher = DefaultReadModelFragmentChangeDispatcher::new(registry);

        let result = dispatcher.dispatch(&envelope(id)).await;

        assert!(result.is_err());
    }
}
