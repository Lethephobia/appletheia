use std::collections::HashSet;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::read_model::list::ReadModelListMatcher;
use crate::read_model::{
    ReadModel, ReadModelFragmentChangeEnvelope, ReadModelNameOwned, SerializedPartition,
};

use super::erased_read_model_watch_session::ErasedReadModelWatchSession;
use super::list_read_model_watch_router::ListReadModelWatchRouter;
use super::snapshot_read_model_watch_router::SnapshotReadModelWatchRouter;
use super::typed_read_model_watch_session::TypedReadModelWatchSession;
use super::{
    ReadModelTypedListWatch, ReadModelWatchDelivery, ReadModelWatchDispatchError,
    ReadModelWatchPartitionState, ReadModelWatchRegistrationError, ReadModelWatchRoute,
    ReadModelWatchSelection, ReadModelWatchSessionId,
};

/// Holds the process-local typed router and delivery endpoint for one watch session.
#[derive(Clone)]
pub struct ReadModelWatchSession {
    read_model_name: ReadModelNameOwned,
    inner: Arc<Mutex<ReadModelWatchSessionState>>,
}

impl ReadModelWatchSession {
    pub(super) fn snapshot<R, D>(session_id: ReadModelWatchSessionId, delivery: D) -> Self
    where
        R: ReadModel + 'static,
        D: ReadModelWatchDelivery,
    {
        let session = TypedReadModelWatchSession::<R, SnapshotReadModelWatchRouter<R>, D>::new(
            session_id,
            SnapshotReadModelWatchRouter::new(),
            delivery,
        );
        Self::new(ReadModelNameOwned::from(R::NAME), Box::new(session))
    }

    pub(super) fn list<R, M, D>(
        session_id: ReadModelWatchSessionId,
        matcher: M,
        watched_list: ReadModelTypedListWatch<M::Query, M::Cursor>,
        delivery: D,
    ) -> Self
    where
        R: ReadModel + 'static,
        M: ReadModelListMatcher + 'static,
        D: ReadModelWatchDelivery,
    {
        let session = TypedReadModelWatchSession::<R, ListReadModelWatchRouter<R, M>, D>::new(
            session_id,
            ListReadModelWatchRouter::new(matcher, watched_list),
            delivery,
        );
        Self::new(ReadModelNameOwned::from(R::NAME), Box::new(session))
    }

    pub(super) async fn replace_selection(
        &self,
        selection: ReadModelWatchSelection,
    ) -> Result<
        (HashSet<SerializedPartition>, HashSet<SerializedPartition>),
        ReadModelWatchRegistrationError,
    > {
        if selection.read_model_name != self.read_model_name {
            return Err(ReadModelWatchRegistrationError::ReadModelMismatch {
                expected: self.read_model_name.clone(),
                actual: selection.read_model_name,
            });
        }
        let mut session_state = self.inner.lock().await;
        let old_partitions = session_state.partition_state.watched_partitions().clone();
        session_state.partition_state = ReadModelWatchPartitionState::new(
            selection.partitions,
            selection.partition_dependencies,
        );
        let new_partitions = session_state.partition_state.watched_partitions().clone();
        Ok((old_partitions, new_partitions))
    }

    pub(super) async fn dispatch(
        &self,
        envelope: &ReadModelFragmentChangeEnvelope,
    ) -> Result<
        (
            ReadModelWatchRoute,
            HashSet<SerializedPartition>,
            HashSet<SerializedPartition>,
        ),
        ReadModelWatchDispatchError,
    > {
        let mut session_state = self.inner.lock().await;
        let old_partitions = session_state.partition_state.watched_partitions().clone();
        let partition_state = session_state.partition_state.clone();
        let route = session_state
            .session
            .dispatch(envelope, &partition_state)
            .await?;
        session_state.partition_state.apply_route(&route);
        let new_partitions = session_state.partition_state.watched_partitions().clone();
        Ok((route, old_partitions, new_partitions))
    }

    fn new(
        read_model_name: ReadModelNameOwned,
        session: Box<dyn ErasedReadModelWatchSession>,
    ) -> Self {
        Self {
            read_model_name,
            inner: Arc::new(Mutex::new(ReadModelWatchSessionState {
                session,
                partition_state: ReadModelWatchPartitionState::default(),
            })),
        }
    }
}

struct ReadModelWatchSessionState {
    session: Box<dyn ErasedReadModelWatchSession>,
    partition_state: ReadModelWatchPartitionState,
}
