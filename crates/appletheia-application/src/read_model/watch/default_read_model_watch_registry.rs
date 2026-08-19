use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

use tokio::sync::{Mutex, Semaphore};

use crate::read_model::{ReadModelDependency, ReadModelInvalidationEnvelope};

use super::read_model_watch_registry_state::{
    ReadModelWatchRegistryState, ReadModelWatchSubscriptionAddress, ReadModelWatchSubscriptionState,
};
use super::{
    ReadModelInvalidationDispatcher, ReadModelListChunkDescriptor, ReadModelListChunkGeneration,
    ReadModelListChunkId, ReadModelWatchCloseReason, ReadModelWatchDelivery, ReadModelWatchEvent,
    ReadModelWatchFailure, ReadModelWatchLimits, ReadModelWatchRefreshError,
    ReadModelWatchRefreshRequest, ReadModelWatchRefreshValue, ReadModelWatchRegistryError,
    ReadModelWatchRevision, ReadModelWatchSessionId, ReadModelWatchSubscriptionExecutor,
    ReadModelWatchSubscriptionId,
};

/// Refreshes and delivers complete snapshots for process-local watch subscriptions.
#[derive(Clone)]
pub struct DefaultReadModelWatchRegistry<D> {
    state: Arc<Mutex<ReadModelWatchRegistryState>>,
    delivery: D,
    limits: ReadModelWatchLimits,
    refresh_permits: Arc<Semaphore>,
    delivery_permits: Arc<Semaphore>,
}

struct PendingReadModelWatchDelivery {
    event: ReadModelWatchEvent,
    value: ReadModelWatchRefreshValue,
    revision: ReadModelWatchRevision,
}

impl<D> DefaultReadModelWatchRegistry<D>
where
    D: ReadModelWatchDelivery + Clone,
{
    pub fn try_new(
        delivery: D,
        limits: ReadModelWatchLimits,
    ) -> Result<Self, ReadModelWatchRegistryError> {
        limits.validate()?;
        Ok(Self {
            state: Arc::new(Mutex::new(ReadModelWatchRegistryState::new())),
            delivery,
            refresh_permits: Arc::new(Semaphore::new(limits.max_concurrent_refreshes)),
            delivery_permits: Arc::new(Semaphore::new(limits.max_concurrent_deliveries)),
            limits,
        })
    }

    /// Opens one transport session.
    pub async fn open_session(&self) -> ReadModelWatchSessionId {
        let session_id = ReadModelWatchSessionId::new();
        self.state
            .lock()
            .await
            .sessions
            .insert(session_id, Instant::now());
        session_id
    }

    /// Extends the idle lifetime of one active transport session.
    pub async fn heartbeat(
        &self,
        session_id: ReadModelWatchSessionId,
    ) -> Result<(), ReadModelWatchRegistryError> {
        let mut state = self.state.lock().await;
        let last_seen = state
            .sessions
            .get_mut(&session_id)
            .ok_or(ReadModelWatchRegistryError::SessionNotFound(session_id))?;
        *last_seen = Instant::now();
        Ok(())
    }

    /// Closes every session whose heartbeat exceeded the configured idle TTL.
    pub async fn expire_idle_sessions(&self) -> Result<usize, ReadModelWatchRegistryError> {
        let now = Instant::now();
        let expired = {
            let state = self.state.lock().await;
            state
                .sessions
                .iter()
                .filter_map(|(session_id, last_seen)| {
                    (now.duration_since(*last_seen) >= self.limits.session_idle_ttl)
                        .then_some(*session_id)
                })
                .collect::<Vec<_>>()
        };
        for session_id in &expired {
            self.close_session(*session_id, ReadModelWatchCloseReason::SessionExpired)
                .await?;
        }
        Ok(expired.len())
    }

    /// Opens a complete single-value snapshot subscription.
    pub async fn subscribe_snapshot<E>(
        &self,
        session_id: ReadModelWatchSessionId,
        prospective_dependencies: impl IntoIterator<Item = ReadModelDependency>,
        executor: E,
    ) -> Result<ReadModelWatchSubscriptionId, ReadModelWatchRegistryError>
    where
        E: ReadModelWatchSubscriptionExecutor,
    {
        self.subscribe(
            session_id,
            prospective_dependencies,
            ReadModelWatchRefreshRequest::Snapshot,
            executor,
        )
        .await
    }

    /// Opens a list subscription and rematerializes its active chunk window.
    pub async fn subscribe_list<E>(
        &self,
        session_id: ReadModelWatchSessionId,
        prospective_dependencies: impl IntoIterator<Item = ReadModelDependency>,
        active_chunks: Vec<ReadModelListChunkDescriptor>,
        executor: E,
    ) -> Result<ReadModelWatchSubscriptionId, ReadModelWatchRegistryError>
    where
        E: ReadModelWatchSubscriptionExecutor,
    {
        if active_chunks.len() > self.limits.max_active_chunks_per_list_subscription {
            return Err(ReadModelWatchRegistryError::ActiveChunkLimitExceeded);
        }
        self.subscribe(
            session_id,
            prospective_dependencies,
            ReadModelWatchRefreshRequest::List { active_chunks },
            executor,
        )
        .await
    }

    async fn subscribe<E>(
        &self,
        session_id: ReadModelWatchSessionId,
        prospective_dependencies: impl IntoIterator<Item = ReadModelDependency>,
        refresh_request: ReadModelWatchRefreshRequest,
        executor: E,
    ) -> Result<ReadModelWatchSubscriptionId, ReadModelWatchRegistryError>
    where
        E: ReadModelWatchSubscriptionExecutor,
    {
        let subscription_id = ReadModelWatchSubscriptionId::new();
        let address = (session_id, subscription_id);
        let prospective_dependencies = prospective_dependencies.into_iter().collect::<HashSet<_>>();
        {
            let mut state = self.state.lock().await;
            if !state.sessions.contains_key(&session_id) {
                return Err(ReadModelWatchRegistryError::SessionNotFound(session_id));
            }
            let subscription_count = state
                .subscriptions
                .keys()
                .filter(|(registered_session_id, _)| registered_session_id == &session_id)
                .count();
            if subscription_count >= self.limits.max_subscriptions_per_session {
                return Err(ReadModelWatchRegistryError::SubscriptionLimitExceeded);
            }
            state.subscriptions.insert(
                address,
                ReadModelWatchSubscriptionState {
                    executor: Arc::new(executor),
                    refresh_request,
                    prospective_dependencies: prospective_dependencies.clone(),
                    materialized_dependencies: HashSet::new(),
                    revision: ReadModelWatchRevision::initial(),
                    last_value: None,
                    refreshing: false,
                    dirty: false,
                },
            );
            state.index(address, prospective_dependencies);
        }

        self.refresh(address).await?;
        Ok(subscription_id)
    }

    /// Registers or replaces one active list chunk, then refreshes the whole active window.
    pub async fn register_list_chunk(
        &self,
        session_id: ReadModelWatchSessionId,
        subscription_id: ReadModelWatchSubscriptionId,
        descriptor: ReadModelListChunkDescriptor,
    ) -> Result<(), ReadModelWatchRegistryError> {
        let address = (session_id, subscription_id);
        {
            let mut state = self.state.lock().await;
            let subscription = state.subscriptions.get_mut(&address).ok_or(
                ReadModelWatchRegistryError::SubscriptionNotFound(subscription_id),
            )?;
            let ReadModelWatchRefreshRequest::List { active_chunks } =
                &mut subscription.refresh_request
            else {
                return Err(ReadModelWatchRegistryError::NotListSubscription);
            };
            if let Some(existing) = active_chunks
                .iter_mut()
                .find(|active| active.chunk_id == descriptor.chunk_id)
            {
                if descriptor.generation <= existing.generation {
                    return Ok(());
                }
                *existing = descriptor;
            } else {
                if active_chunks.len() >= self.limits.max_active_chunks_per_list_subscription {
                    return Err(ReadModelWatchRegistryError::ActiveChunkLimitExceeded);
                }
                active_chunks.push(descriptor);
            }
        }
        self.refresh(address).await
    }

    /// Releases one active list chunk when its generation still matches.
    pub async fn release_list_chunk(
        &self,
        session_id: ReadModelWatchSessionId,
        subscription_id: ReadModelWatchSubscriptionId,
        chunk_id: ReadModelListChunkId,
        generation: ReadModelListChunkGeneration,
    ) -> Result<(), ReadModelWatchRegistryError> {
        let address = (session_id, subscription_id);
        let removed = {
            let mut state = self.state.lock().await;
            let subscription = state.subscriptions.get_mut(&address).ok_or(
                ReadModelWatchRegistryError::SubscriptionNotFound(subscription_id),
            )?;
            let ReadModelWatchRefreshRequest::List { active_chunks } =
                &mut subscription.refresh_request
            else {
                return Err(ReadModelWatchRegistryError::NotListSubscription);
            };
            let old_len = active_chunks.len();
            active_chunks
                .retain(|active| active.chunk_id != chunk_id || active.generation != generation);
            old_len != active_chunks.len()
        };
        if removed {
            self.refresh(address).await?;
        }
        Ok(())
    }

    /// Refreshes every subscription affected by an internal invalidation.
    pub async fn invalidate(
        &self,
        envelope: &ReadModelInvalidationEnvelope,
    ) -> Result<(), ReadModelWatchRegistryError> {
        let addresses = {
            let state = self.state.lock().await;
            envelope
                .invalidated_dependencies
                .iter()
                .filter_map(|dependency| state.subscriptions_by_dependency.get(dependency))
                .flatten()
                .copied()
                .collect::<HashSet<_>>()
        };
        for address in addresses {
            self.refresh(address).await?;
        }
        Ok(())
    }

    /// Releases one logical subscription and its dependency-index entries.
    pub async fn release(
        &self,
        session_id: ReadModelWatchSessionId,
        subscription_id: ReadModelWatchSubscriptionId,
    ) -> Result<(), ReadModelWatchRegistryError> {
        let address = (session_id, subscription_id);
        let removed = {
            let mut state = self.state.lock().await;
            let Some(subscription) = state.subscriptions.remove(&address) else {
                return Ok(());
            };
            let dependencies = subscription
                .prospective_dependencies
                .into_iter()
                .chain(subscription.materialized_dependencies)
                .collect::<HashSet<_>>();
            state.unindex(&address, dependencies);
            true
        };
        if removed {
            self.deliver_event(
                session_id,
                ReadModelWatchEvent::SubscriptionClosed {
                    subscription_id,
                    reason: ReadModelWatchCloseReason::ClientReleased,
                },
            )
            .await?;
        }
        Ok(())
    }

    /// Closes a session and atomically removes all of its subscriptions from the index.
    pub async fn close_session(
        &self,
        session_id: ReadModelWatchSessionId,
        reason: ReadModelWatchCloseReason,
    ) -> Result<(), ReadModelWatchRegistryError> {
        let subscription_ids = {
            let mut state = self.state.lock().await;
            state.sessions.remove(&session_id);
            let subscription_ids = state
                .subscriptions
                .keys()
                .filter_map(|(registered_session_id, subscription_id)| {
                    (registered_session_id == &session_id).then_some(*subscription_id)
                })
                .collect::<Vec<_>>();
            for subscription_id in &subscription_ids {
                let address = (session_id, *subscription_id);
                if let Some(subscription) = state.subscriptions.remove(&address) {
                    let dependencies = subscription
                        .prospective_dependencies
                        .into_iter()
                        .chain(subscription.materialized_dependencies)
                        .collect::<HashSet<_>>();
                    state.unindex(&address, dependencies);
                }
            }
            subscription_ids
        };
        for subscription_id in subscription_ids {
            self.deliver_event(
                session_id,
                ReadModelWatchEvent::SubscriptionClosed {
                    subscription_id,
                    reason,
                },
            )
            .await?;
        }
        Ok(())
    }

    async fn refresh(
        &self,
        address: ReadModelWatchSubscriptionAddress,
    ) -> Result<(), ReadModelWatchRegistryError> {
        let executor = {
            let mut state = self.state.lock().await;
            let Some(subscription) = state.subscriptions.get_mut(&address) else {
                return Ok(());
            };
            if subscription.refreshing {
                subscription.dirty = true;
                return Ok(());
            }
            subscription.refreshing = true;
            Arc::clone(&subscription.executor)
        };

        loop {
            let refresh_request = {
                let state = self.state.lock().await;
                let Some(subscription) = state.subscriptions.get(&address) else {
                    return Ok(());
                };
                subscription.refresh_request.clone()
            };
            let refresh_permit = Arc::clone(&self.refresh_permits)
                .acquire_owned()
                .await
                .map_err(|_| ReadModelWatchRegistryError::RefreshSchedulerClosed)?;
            let refresh_result = executor.refresh(refresh_request).await;
            drop(refresh_permit);
            let (event, pending_delivery, rerun) = match refresh_result {
                Ok(refresh) => self.apply_refresh(address, refresh).await?,
                Err(ReadModelWatchRefreshError::Failed(failure)) => {
                    let (event, rerun) = self.finish_failed_refresh(address, failure).await;
                    (event, None, rerun)
                }
                Err(ReadModelWatchRefreshError::Closed(reason)) => {
                    let (event, rerun) = self.finish_closed_refresh(address, reason).await;
                    (event, None, rerun)
                }
            };
            if let Some(event) = event
                && let Err(error) = self.deliver_event(address.0, event).await
            {
                if pending_delivery.is_some() {
                    self.finish_delivery_failure(address).await;
                }
                return Err(error);
            }
            let rerun = if let Some(pending_delivery) = pending_delivery {
                self.finish_successful_delivery(address, pending_delivery)
                    .await
            } else {
                rerun
            };
            if !rerun {
                return Ok(());
            }
        }
    }

    async fn apply_refresh(
        &self,
        address: ReadModelWatchSubscriptionAddress,
        refresh: super::ReadModelWatchRefresh,
    ) -> Result<
        (
            Option<ReadModelWatchEvent>,
            Option<PendingReadModelWatchDelivery>,
            bool,
        ),
        ReadModelWatchRegistryError,
    > {
        let mut state = self.state.lock().await;
        if state
            .subscriptions
            .get(&address)
            .is_some_and(|subscription| subscription.dirty)
        {
            if let Some(subscription) = state.subscriptions.get_mut(&address) {
                subscription.dirty = false;
                subscription.refreshing = true;
            }
            return Ok((None, None, true));
        }
        if matches!(
            &refresh.value,
            ReadModelWatchRefreshValue::List(chunks)
                if chunks.len() > self.limits.max_active_chunks_per_list_subscription
        ) {
            let subscription = state.subscriptions.get_mut(&address);
            if let Some(subscription) = subscription {
                subscription.refreshing = false;
                subscription.dirty = false;
            }
            return Ok((
                Some(ReadModelWatchEvent::SubscriptionError {
                    subscription_id: address.1,
                    failure: ReadModelWatchFailure {
                        code: "active_chunk_limit_exceeded".to_owned(),
                        retryable: false,
                    },
                }),
                None,
                false,
            ));
        }
        let new_materialized = refresh
            .materialized_dependencies
            .into_iter()
            .collect::<HashSet<_>>();
        if new_materialized.len() > self.limits.max_materialized_dependencies_per_subscription {
            let subscription = state.subscriptions.get_mut(&address);
            if let Some(subscription) = subscription {
                subscription.refreshing = false;
                subscription.dirty = false;
            }
            return Ok((
                Some(ReadModelWatchEvent::SubscriptionError {
                    subscription_id: address.1,
                    failure: ReadModelWatchFailure {
                        code: "materialized_dependency_limit_exceeded".to_owned(),
                        retryable: false,
                    },
                }),
                None,
                false,
            ));
        }

        let (old_materialized, prospective, pending_delivery, rerun) = {
            let Some(subscription) = state.subscriptions.get_mut(&address) else {
                return Ok((None, None, false));
            };
            let old_materialized = std::mem::replace(
                &mut subscription.materialized_dependencies,
                new_materialized.clone(),
            );
            let prospective = subscription.prospective_dependencies.clone();
            let changed = subscription.last_value.as_ref() != Some(&refresh.value);
            let pending_delivery = if changed {
                let next_revision = subscription
                    .revision
                    .checked_next()
                    .ok_or(ReadModelWatchRegistryError::RevisionOverflow)?;
                let event = match refresh.value.clone() {
                    ReadModelWatchRefreshValue::Snapshot(value) => {
                        ReadModelWatchEvent::SnapshotUpdated {
                            subscription_id: address.1,
                            revision: next_revision,
                            value,
                        }
                    }
                    ReadModelWatchRefreshValue::List(chunks) => {
                        ReadModelWatchEvent::ListSnapshotUpdated {
                            subscription_id: address.1,
                            revision: next_revision,
                            chunks,
                        }
                    }
                };
                Some(PendingReadModelWatchDelivery {
                    event,
                    value: refresh.value,
                    revision: next_revision,
                })
            } else {
                None
            };
            let rerun = if pending_delivery.is_some() {
                false
            } else {
                let rerun = std::mem::take(&mut subscription.dirty);
                subscription.refreshing = rerun;
                rerun
            };
            (old_materialized, prospective, pending_delivery, rerun)
        };
        let old_effective = old_materialized
            .into_iter()
            .chain(prospective.iter().cloned())
            .collect::<HashSet<_>>();
        let new_effective = new_materialized
            .iter()
            .cloned()
            .chain(prospective)
            .collect::<HashSet<_>>();
        state.unindex(&address, old_effective.difference(&new_effective).cloned());
        state.index(address, new_effective.difference(&old_effective).cloned());

        let event = pending_delivery
            .as_ref()
            .map(|pending| pending.event.clone());
        Ok((event, pending_delivery, rerun))
    }

    async fn finish_successful_delivery(
        &self,
        address: ReadModelWatchSubscriptionAddress,
        pending: PendingReadModelWatchDelivery,
    ) -> bool {
        let mut state = self.state.lock().await;
        let Some(subscription) = state.subscriptions.get_mut(&address) else {
            return false;
        };
        subscription.revision = pending.revision;
        subscription.last_value = Some(pending.value);
        let rerun = std::mem::take(&mut subscription.dirty);
        subscription.refreshing = rerun;
        rerun
    }

    async fn finish_delivery_failure(&self, address: ReadModelWatchSubscriptionAddress) {
        if let Some(subscription) = self.state.lock().await.subscriptions.get_mut(&address) {
            subscription.refreshing = false;
        }
    }

    async fn finish_failed_refresh(
        &self,
        address: ReadModelWatchSubscriptionAddress,
        failure: ReadModelWatchFailure,
    ) -> (Option<ReadModelWatchEvent>, bool) {
        let mut state = self.state.lock().await;
        let Some(subscription) = state.subscriptions.get_mut(&address) else {
            return (None, false);
        };
        let rerun = std::mem::take(&mut subscription.dirty);
        subscription.refreshing = rerun;
        let event = (!rerun).then_some(ReadModelWatchEvent::SubscriptionError {
            subscription_id: address.1,
            failure,
        });
        (event, rerun)
    }

    async fn finish_closed_refresh(
        &self,
        address: ReadModelWatchSubscriptionAddress,
        reason: ReadModelWatchCloseReason,
    ) -> (Option<ReadModelWatchEvent>, bool) {
        let mut state = self.state.lock().await;
        let Some(subscription) = state.subscriptions.remove(&address) else {
            return (None, false);
        };
        let dependencies = subscription
            .prospective_dependencies
            .into_iter()
            .chain(subscription.materialized_dependencies)
            .collect::<HashSet<_>>();
        state.unindex(&address, dependencies);
        (
            Some(ReadModelWatchEvent::SubscriptionClosed {
                subscription_id: address.1,
                reason,
            }),
            false,
        )
    }

    async fn deliver_event(
        &self,
        session_id: ReadModelWatchSessionId,
        event: ReadModelWatchEvent,
    ) -> Result<(), ReadModelWatchRegistryError> {
        let delivery_permit = Arc::clone(&self.delivery_permits)
            .try_acquire_owned()
            .map_err(|_| ReadModelWatchRegistryError::DeliveryBackpressureExceeded)?;
        let result = self.delivery.deliver(&session_id, &event).await;
        drop(delivery_permit);
        result.map_err(Into::into)
    }
}

impl<D> ReadModelInvalidationDispatcher for DefaultReadModelWatchRegistry<D>
where
    D: ReadModelWatchDelivery + Clone,
{
    async fn dispatch(
        &self,
        envelope: &ReadModelInvalidationEnvelope,
    ) -> Result<(), ReadModelWatchRegistryError> {
        self.invalidate(envelope).await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::io;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex as StdMutex};
    use std::time::Duration;

    use appletheia_domain::{AggregateVersion, EventId, EventOccurredAt};
    use serde_json::json;
    use tokio::sync::{Notify, Semaphore};
    use uuid::Uuid;

    use crate::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
        SerializedEventPayload,
    };
    use crate::projection::ProjectorName;
    use crate::read_model::pagination::{CursorWindow, PageSize};
    use crate::read_model::{ReadModelInvalidationEnvelope, SerializedPartition};
    use crate::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };

    use super::*;
    use crate::read_model::watch::{
        ReadModelWatchDeliveryError, ReadModelWatchRefresh, ReadModelWatchRefreshFuture,
        SerializedReadModelListChunk, SerializedReadModelSnapshot,
    };

    #[derive(Clone, Default)]
    struct TestDelivery {
        events: Arc<StdMutex<Vec<(ReadModelWatchSessionId, ReadModelWatchEvent)>>>,
    }

    impl ReadModelWatchDelivery for TestDelivery {
        async fn deliver(
            &self,
            session_id: &ReadModelWatchSessionId,
            event: &ReadModelWatchEvent,
        ) -> Result<(), ReadModelWatchDeliveryError> {
            self.events
                .lock()
                .map_err(|_| {
                    ReadModelWatchDeliveryError::new(io::Error::other(
                        "test delivery lock poisoned",
                    ))
                })?
                .push((*session_id, event.clone()));
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct FailOnceDelivery {
        attempts: Arc<AtomicUsize>,
        events: Arc<StdMutex<Vec<ReadModelWatchEvent>>>,
    }

    impl ReadModelWatchDelivery for FailOnceDelivery {
        async fn deliver(
            &self,
            _session_id: &ReadModelWatchSessionId,
            event: &ReadModelWatchEvent,
        ) -> Result<(), ReadModelWatchDeliveryError> {
            if self.attempts.fetch_add(1, Ordering::SeqCst) == 0 {
                return Err(ReadModelWatchDeliveryError::new(io::Error::other(
                    "transient test delivery failure",
                )));
            }
            self.events
                .lock()
                .map_err(|_| {
                    ReadModelWatchDeliveryError::new(io::Error::other(
                        "test delivery lock poisoned",
                    ))
                })?
                .push(event.clone());
            Ok(())
        }
    }

    struct TestExecutor {
        refreshes: Arc<StdMutex<VecDeque<ReadModelWatchRefresh>>>,
    }

    impl TestExecutor {
        fn new(refreshes: impl IntoIterator<Item = ReadModelWatchRefresh>) -> Self {
            Self {
                refreshes: Arc::new(StdMutex::new(refreshes.into_iter().collect())),
            }
        }
    }

    impl ReadModelWatchSubscriptionExecutor for TestExecutor {
        fn refresh(
            &self,
            _request: ReadModelWatchRefreshRequest,
        ) -> ReadModelWatchRefreshFuture<'_> {
            let refresh = self
                .refreshes
                .lock()
                .map_err(|_| ReadModelWatchFailure {
                    code: "test_executor_lock_poisoned".to_owned(),
                    retryable: false,
                })
                .and_then(|mut refreshes| {
                    refreshes.pop_front().ok_or(ReadModelWatchFailure {
                        code: "missing_test_refresh".to_owned(),
                        retryable: false,
                    })
                });
            Box::pin(async move { refresh.map_err(ReadModelWatchRefreshError::Failed) })
        }
    }

    #[derive(Clone)]
    struct DirtyExecutor {
        calls: Arc<AtomicUsize>,
        dependency: ReadModelDependency,
        refresh_started: Arc<Notify>,
        refresh_release: Arc<Semaphore>,
    }

    impl ReadModelWatchSubscriptionExecutor for DirtyExecutor {
        fn refresh(
            &self,
            _request: ReadModelWatchRefreshRequest,
        ) -> ReadModelWatchRefreshFuture<'_> {
            let call = self.calls.fetch_add(1, Ordering::SeqCst);
            Box::pin(async move {
                if call == 1 {
                    self.refresh_started.notify_one();
                    self.refresh_release
                        .acquire()
                        .await
                        .map_err(|_| {
                            ReadModelWatchRefreshError::Failed(ReadModelWatchFailure {
                                code: "test_refresh_cancelled".to_owned(),
                                retryable: false,
                            })
                        })?
                        .forget();
                }
                Ok(refresh(call + 1, vec![self.dependency.clone()]))
            })
        }
    }

    struct AuthorizationRevokedExecutor {
        calls: AtomicUsize,
        dependency: ReadModelDependency,
    }

    impl ReadModelWatchSubscriptionExecutor for AuthorizationRevokedExecutor {
        fn refresh(
            &self,
            _request: ReadModelWatchRefreshRequest,
        ) -> ReadModelWatchRefreshFuture<'_> {
            let call = self.calls.fetch_add(1, Ordering::SeqCst);
            Box::pin(async move {
                if call == 0 {
                    Ok(refresh(1, vec![self.dependency.clone()]))
                } else {
                    Err(ReadModelWatchRefreshError::Closed(
                        ReadModelWatchCloseReason::AuthorizationDenied,
                    ))
                }
            })
        }
    }

    #[derive(Clone, Default)]
    struct ListExecutor {
        requests: Arc<StdMutex<Vec<ReadModelWatchRefreshRequest>>>,
    }

    impl ReadModelWatchSubscriptionExecutor for ListExecutor {
        fn refresh(
            &self,
            request: ReadModelWatchRefreshRequest,
        ) -> ReadModelWatchRefreshFuture<'_> {
            let chunks = match &request {
                ReadModelWatchRefreshRequest::Snapshot => {
                    return Box::pin(async {
                        Err(ReadModelWatchRefreshError::Failed(ReadModelWatchFailure {
                            code: "unexpected_snapshot_request".to_owned(),
                            retryable: false,
                        }))
                    });
                }
                ReadModelWatchRefreshRequest::List { active_chunks } => active_chunks
                    .iter()
                    .map(|descriptor| SerializedReadModelListChunk {
                        chunk_id: descriptor.chunk_id,
                        generation: descriptor.generation,
                        items: Vec::new(),
                        start_cursor: None,
                        end_cursor: None,
                        has_previous: false,
                        has_next: false,
                    })
                    .collect::<Vec<_>>(),
            };
            let stored_request = match self.requests.lock() {
                Ok(mut requests) => {
                    requests.push(request);
                    Ok(())
                }
                Err(_) => Err(ReadModelWatchRefreshError::Failed(ReadModelWatchFailure {
                    code: "test_request_lock_poisoned".to_owned(),
                    retryable: false,
                })),
            };
            Box::pin(async move {
                stored_request?;
                Ok(ReadModelWatchRefresh {
                    value: ReadModelWatchRefreshValue::List(chunks),
                    materialized_dependencies: Vec::new(),
                })
            })
        }
    }

    fn dependency(value: &str) -> ReadModelDependency {
        ReadModelDependency::Partition(
            SerializedPartition::try_from(json!({ "fragment": "test", "key": value }))
                .expect("partition should be valid"),
        )
    }

    fn refresh(value: usize, dependencies: Vec<ReadModelDependency>) -> ReadModelWatchRefresh {
        ReadModelWatchRefresh {
            value: ReadModelWatchRefreshValue::Snapshot(SerializedReadModelSnapshot::from(
                json!({ "value": value }),
            )),
            materialized_dependencies: dependencies,
        }
    }

    fn event_envelope() -> EventEnvelope {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let message_id = MessageId::new();
        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::try_from("watch_test")
                .expect("aggregate type should be valid"),
            aggregate_id: AggregateIdValue::from(Uuid::now_v7()),
            aggregate_version: AggregateVersion::try_from(1)
                .expect("aggregate version should be valid"),
            event_name: EventNameOwned::try_from("changed").expect("event name should be valid"),
            payload: SerializedEventPayload::try_from(json!({})).expect("payload should be valid"),
            occurred_at: EventOccurredAt::now(),
            correlation_id,
            causation_id: CausationId::from(message_id),
            context: RequestContext::new(correlation_id, message_id, Principal::System)
                .expect("request context should be valid"),
        }
    }

    fn invalidation(dependency: ReadModelDependency) -> ReadModelInvalidationEnvelope {
        ReadModelInvalidationEnvelope::try_new(
            &event_envelope(),
            ProjectorName::new("watch_test_projector"),
            [dependency],
        )
        .expect("invalidation should be valid")
    }

    #[tokio::test]
    async fn delivers_complete_snapshots_with_monotonic_revisions() {
        let watched_dependency = dependency("one");
        let delivery = TestDelivery::default();
        let registry = DefaultReadModelWatchRegistry::try_new(
            delivery.clone(),
            ReadModelWatchLimits::default(),
        )
        .expect("limits should be valid");
        let session_id = registry.open_session().await;
        let subscription_id = registry
            .subscribe_snapshot(
                session_id,
                [watched_dependency.clone()],
                TestExecutor::new([
                    refresh(1, vec![watched_dependency.clone()]),
                    refresh(2, vec![watched_dependency.clone()]),
                ]),
            )
            .await
            .expect("subscription should open");

        registry
            .invalidate(&invalidation(watched_dependency))
            .await
            .expect("invalidation should refresh");

        let events = delivery.events.lock().expect("events should lock");
        assert_eq!(events.len(), 2);
        assert!(matches!(
            &events[0].1,
            ReadModelWatchEvent::SnapshotUpdated {
                subscription_id: actual_subscription_id,
                revision,
                ..
            } if actual_subscription_id == &subscription_id && revision.value() == 1
        ));
        assert!(matches!(
            &events[1].1,
            ReadModelWatchEvent::SnapshotUpdated { revision, .. }
                if revision.value() == 2
        ));
    }

    #[tokio::test]
    async fn retries_a_snapshot_after_delivery_failure_without_advancing_revision() {
        let watched_dependency = dependency("one");
        let delivery = FailOnceDelivery::default();
        let registry = DefaultReadModelWatchRegistry::try_new(
            delivery.clone(),
            ReadModelWatchLimits::default(),
        )
        .expect("limits should be valid");
        let session_id = registry.open_session().await;
        let result = registry
            .subscribe_snapshot(
                session_id,
                [watched_dependency.clone()],
                TestExecutor::new([
                    refresh(1, vec![watched_dependency.clone()]),
                    refresh(1, vec![watched_dependency.clone()]),
                ]),
            )
            .await;
        assert!(matches!(
            result,
            Err(ReadModelWatchRegistryError::Delivery(_))
        ));

        registry
            .invalidate(&invalidation(watched_dependency))
            .await
            .expect("the undelivered snapshot should remain eligible for delivery");

        let events = delivery.events.lock().expect("events should lock");
        assert!(matches!(
            events.as_slice(),
            [ReadModelWatchEvent::SnapshotUpdated { revision, .. }]
                if revision.value() == 1
        ));
    }

    #[tokio::test]
    async fn suppresses_unchanged_snapshots_and_replaces_materialized_dependencies() {
        let prospective_dependency = dependency("prospective");
        let first_materialized = dependency("first");
        let second_materialized = dependency("second");
        let delivery = TestDelivery::default();
        let registry = DefaultReadModelWatchRegistry::try_new(
            delivery.clone(),
            ReadModelWatchLimits::default(),
        )
        .expect("limits should be valid");
        let session_id = registry.open_session().await;
        registry
            .subscribe_snapshot(
                session_id,
                [prospective_dependency],
                TestExecutor::new([
                    refresh(1, vec![first_materialized.clone()]),
                    refresh(1, vec![second_materialized.clone()]),
                ]),
            )
            .await
            .expect("subscription should open");

        registry
            .invalidate(&invalidation(first_materialized.clone()))
            .await
            .expect("first dependency should refresh");
        registry
            .invalidate(&invalidation(first_materialized))
            .await
            .expect("removed dependency should be ignored");

        assert_eq!(delivery.events.lock().expect("events should lock").len(), 1);
        let state = registry.state.lock().await;
        assert!(
            !state
                .subscriptions_by_dependency
                .contains_key(&dependency("first"))
        );
        assert!(
            state
                .subscriptions_by_dependency
                .contains_key(&second_materialized)
        );
    }

    #[tokio::test]
    async fn closing_a_session_removes_dependencies_and_delivers_closed() {
        let watched_dependency = dependency("one");
        let delivery = TestDelivery::default();
        let registry = DefaultReadModelWatchRegistry::try_new(
            delivery.clone(),
            ReadModelWatchLimits::default(),
        )
        .expect("limits should be valid");
        let session_id = registry.open_session().await;
        registry
            .subscribe_snapshot(
                session_id,
                [watched_dependency.clone()],
                TestExecutor::new([refresh(1, vec![watched_dependency])]),
            )
            .await
            .expect("subscription should open");

        registry
            .close_session(session_id, ReadModelWatchCloseReason::SessionClosed)
            .await
            .expect("session should close");

        let state = registry.state.lock().await;
        assert!(state.subscriptions.is_empty());
        assert!(state.subscriptions_by_dependency.is_empty());
        drop(state);
        assert!(matches!(
            delivery
                .events
                .lock()
                .expect("events should lock")
                .last()
                .map(|(_, event)| event),
            Some(ReadModelWatchEvent::SubscriptionClosed {
                reason: ReadModelWatchCloseReason::SessionClosed,
                ..
            })
        ));
    }

    #[tokio::test]
    async fn coalesces_an_invalidation_during_refresh_into_one_dirty_rerun() {
        let watched_dependency = dependency("one");
        let delivery = TestDelivery::default();
        let registry = DefaultReadModelWatchRegistry::try_new(
            delivery.clone(),
            ReadModelWatchLimits::default(),
        )
        .expect("limits should be valid");
        let session_id = registry.open_session().await;
        let executor = DirtyExecutor {
            calls: Arc::new(AtomicUsize::new(0)),
            dependency: watched_dependency.clone(),
            refresh_started: Arc::new(Notify::new()),
            refresh_release: Arc::new(Semaphore::new(0)),
        };
        registry
            .subscribe_snapshot(session_id, [watched_dependency.clone()], executor.clone())
            .await
            .expect("subscription should open");

        let first_invalidation = invalidation(watched_dependency.clone());
        let first_refresh = registry.invalidate(&first_invalidation);
        let concurrent_invalidation = async {
            executor.refresh_started.notified().await;
            registry
                .invalidate(&invalidation(watched_dependency))
                .await
                .expect("concurrent invalidation should mark the refresh dirty");
            executor.refresh_release.add_permits(1);
        };
        let (first_result, ()) = tokio::join!(first_refresh, concurrent_invalidation);
        first_result.expect("dirty refresh should succeed");

        assert_eq!(executor.calls.load(Ordering::SeqCst), 3);
        let events = delivery.events.lock().expect("events should lock");
        assert_eq!(events.len(), 2);
        assert!(matches!(
            &events[1].1,
            ReadModelWatchEvent::SnapshotUpdated { revision, .. }
                if revision.value() == 2
        ));
    }

    #[tokio::test]
    async fn replaces_and_releases_list_chunks_by_generation() {
        let delivery = TestDelivery::default();
        let registry = DefaultReadModelWatchRegistry::try_new(
            delivery.clone(),
            ReadModelWatchLimits::default(),
        )
        .expect("limits should be valid");
        let session_id = registry.open_session().await;
        let chunk_id = ReadModelListChunkId::new();
        let first_generation = ReadModelListChunkGeneration::initial();
        let second_generation = first_generation
            .checked_next()
            .expect("generation should increment");
        let limit = PageSize::try_from(20).expect("page size should be valid");
        let first_descriptor = ReadModelListChunkDescriptor {
            chunk_id,
            generation: first_generation,
            window: CursorWindow::Forward { after: None, limit },
        };
        let executor = ListExecutor::default();
        let subscription_id = registry
            .subscribe_list(
                session_id,
                Vec::new(),
                vec![first_descriptor.clone()],
                executor.clone(),
            )
            .await
            .expect("list subscription should open");

        registry
            .register_list_chunk(session_id, subscription_id, first_descriptor)
            .await
            .expect("same generation should be idempotent");
        registry
            .register_list_chunk(
                session_id,
                subscription_id,
                ReadModelListChunkDescriptor {
                    chunk_id,
                    generation: second_generation,
                    window: CursorWindow::Backward {
                        before: None,
                        limit,
                    },
                },
            )
            .await
            .expect("new generation should replace the chunk");
        registry
            .release_list_chunk(session_id, subscription_id, chunk_id, first_generation)
            .await
            .expect("stale release should be ignored");
        registry
            .release_list_chunk(session_id, subscription_id, chunk_id, second_generation)
            .await
            .expect("current generation should release");

        assert_eq!(
            executor
                .requests
                .lock()
                .expect("requests should lock")
                .len(),
            3
        );
        let events = delivery.events.lock().expect("events should lock");
        assert_eq!(events.len(), 3);
        assert!(matches!(
            &events[2].1,
            ReadModelWatchEvent::ListSnapshotUpdated { revision, chunks, .. }
                if revision.value() == 3 && chunks.is_empty()
        ));
    }

    #[tokio::test]
    async fn expires_idle_sessions_and_cleans_the_dependency_index() {
        let watched_dependency = dependency("one");
        let delivery = TestDelivery::default();
        let registry = DefaultReadModelWatchRegistry::try_new(
            delivery.clone(),
            ReadModelWatchLimits {
                session_idle_ttl: Duration::from_secs(1),
                ..ReadModelWatchLimits::default()
            },
        )
        .expect("limits should be valid");
        let session_id = registry.open_session().await;
        registry
            .subscribe_snapshot(
                session_id,
                [watched_dependency.clone()],
                TestExecutor::new([refresh(1, vec![watched_dependency])]),
            )
            .await
            .expect("subscription should open");
        registry
            .state
            .lock()
            .await
            .sessions
            .insert(session_id, Instant::now() - Duration::from_secs(2));

        let expired = registry
            .expire_idle_sessions()
            .await
            .expect("idle sessions should expire");

        assert_eq!(expired, 1);
        let state = registry.state.lock().await;
        assert!(state.sessions.is_empty());
        assert!(state.subscriptions.is_empty());
        assert!(state.subscriptions_by_dependency.is_empty());
        drop(state);
        assert!(matches!(
            delivery
                .events
                .lock()
                .expect("events should lock")
                .last()
                .map(|(_, event)| event),
            Some(ReadModelWatchEvent::SubscriptionClosed {
                reason: ReadModelWatchCloseReason::SessionExpired,
                ..
            })
        ));
    }

    #[tokio::test]
    async fn authorization_revocation_closes_and_unindexes_the_subscription() {
        let watched_dependency = dependency("one");
        let delivery = TestDelivery::default();
        let registry = DefaultReadModelWatchRegistry::try_new(
            delivery.clone(),
            ReadModelWatchLimits::default(),
        )
        .expect("limits should be valid");
        let session_id = registry.open_session().await;
        registry
            .subscribe_snapshot(
                session_id,
                [watched_dependency.clone()],
                AuthorizationRevokedExecutor {
                    calls: AtomicUsize::new(0),
                    dependency: watched_dependency.clone(),
                },
            )
            .await
            .expect("subscription should open");

        registry
            .invalidate(&invalidation(watched_dependency))
            .await
            .expect("authorization closure should be delivered");

        let state = registry.state.lock().await;
        assert!(state.subscriptions.is_empty());
        assert!(state.subscriptions_by_dependency.is_empty());
        drop(state);
        assert!(matches!(
            delivery
                .events
                .lock()
                .expect("events should lock")
                .last()
                .map(|(_, event)| event),
            Some(ReadModelWatchEvent::SubscriptionClosed {
                reason: ReadModelWatchCloseReason::AuthorizationDenied,
                ..
            })
        ));
    }
}
