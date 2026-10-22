use std::{collections::HashMap, sync::Mutex};

use tokio_util::sync::CancellationToken;

use tokio::sync::watch;

use crate::{
    messaging::{Consumer, ConsumerGroup, Delivery, Subscriber, Subscription},
    unit_of_work::{UnitOfWork, UnitOfWorkFactory},
};

use super::{
    DefaultReadModelWatchReceiver, ReadModelWatchEndpoint, ReadModelWatchEndpointConfig,
    ReadModelWatchEndpointError, ReadModelWatchEndpointId, ReadModelWatchEndpointLeaseExpiresAt,
    ReadModelWatchEndpointRegistry, ReadModelWatchId, ReadModelWatchLookup,
    ReadModelWatchNotificationEnvelope, ReadModelWatchNotificationSelector,
    default_read_model_watch_endpoint_run_guard::DefaultReadModelWatchEndpointRunGuard,
    default_read_model_watch_endpoint_state::DefaultReadModelWatchEndpointState,
    default_read_model_watch_endpoint_status::DefaultReadModelWatchEndpointStatus,
};

/// Routes endpoint notifications to local watch receivers and maintains the endpoint lease.
/// Run once per instance. If the run future fails or is cancelled, local receivers close;
/// persisted registrations become inactive when the endpoint lease expires.
pub struct DefaultReadModelWatchEndpoint<F, ER, L, S>
where
    F: UnitOfWorkFactory,
    ER: ReadModelWatchEndpointRegistry<Uow = F::Uow>,
    L: ReadModelWatchLookup<Uow = F::Uow>,
    S: Subscriber<
            ReadModelWatchNotificationEnvelope,
            Selector = ReadModelWatchNotificationSelector,
        >,
{
    unit_of_work_factory: F,
    endpoint_registry: ER,
    watch_lookup: L,
    subscriber: S,
    config: ReadModelWatchEndpointConfig,
    endpoint_id: ReadModelWatchEndpointId,
    state: Mutex<DefaultReadModelWatchEndpointState>,
    stop_token: CancellationToken,
}

impl<F, ER, L, S> DefaultReadModelWatchEndpoint<F, ER, L, S>
where
    F: UnitOfWorkFactory,
    ER: ReadModelWatchEndpointRegistry<Uow = F::Uow>,
    L: ReadModelWatchLookup<Uow = F::Uow>,
    S: Subscriber<
            ReadModelWatchNotificationEnvelope,
            Selector = ReadModelWatchNotificationSelector,
        >,
{
    pub fn new(
        unit_of_work_factory: F,
        endpoint_registry: ER,
        watch_lookup: L,
        subscriber: S,
        config: ReadModelWatchEndpointConfig,
    ) -> Self {
        let (status, _) = watch::channel(DefaultReadModelWatchEndpointStatus::Idle);
        Self {
            unit_of_work_factory,
            endpoint_registry,
            watch_lookup,
            subscriber,
            config,
            endpoint_id: ReadModelWatchEndpointId::new(),
            stop_token: CancellationToken::new(),
            state: Mutex::new(DefaultReadModelWatchEndpointState {
                status,
                watch_senders: HashMap::new(),
            }),
        }
    }

    async fn receive_notifications(
        &self,
        mut consumer: S::Consumer,
    ) -> Result<(), ReadModelWatchEndpointError> {
        while !self.is_stop_requested() {
            let mut delivery = tokio::select! {
                biased;
                _ = self.stop_token.cancelled() => break,
                delivery = consumer.next() => delivery?,
            };
            let notification = &delivery.message().notification;
            if notification.endpoint_id != self.endpoint_id {
                delivery.ack().await?;
                continue;
            }
            let result: Result<(), ReadModelWatchEndpointError> = async {
                let mut cursor = None;
                loop {
                    let mut uow = self.unit_of_work_factory.begin().await?;
                    let page = match self
                        .watch_lookup
                        .find_watch_ids_by_target(
                            &mut uow,
                            self.endpoint_id,
                            &notification.target,
                            cursor,
                            self.config.watch_page_size(),
                        )
                        .await
                    {
                        Ok(page) => page,
                        Err(source) => {
                            return Err(uow
                                .rollback_with_operation_error(ReadModelWatchEndpointError::from(
                                    source,
                                ))
                                .await?);
                        }
                    };
                    uow.commit().await?;
                    {
                        let state = self
                            .state
                            .lock()
                            .map_err(|_| ReadModelWatchEndpointError::StatePoisoned)?;
                        for watch_id in &page.watch_ids {
                            if let Some(sender) = state.watch_senders.get(watch_id) {
                                let _ = sender.send(());
                            }
                        }
                    }
                    cursor = page.next_cursor;
                    if cursor.is_none() {
                        break;
                    }
                }
                Ok(())
            }
            .await;
            match result {
                Ok(()) => delivery.ack().await?,
                Err(error) => {
                    delivery.nack().await?;
                    return Err(error);
                }
            }
        }
        Ok(())
    }

    async fn refresh_endpoint_lease(&self) -> Result<(), ReadModelWatchEndpointError> {
        let interval = self.config.endpoint_lease_refresh_interval().to_std();
        while !self.is_stop_requested() {
            tokio::select! {
                biased;
                _ = self.stop_token.cancelled() => break,
                _ = tokio::time::sleep(interval) => {}
            }
            let mut uow = self.unit_of_work_factory.begin().await?;
            let lease_until = ReadModelWatchEndpointLeaseExpiresAt::from_now(
                self.config.endpoint_lease_duration(),
            );
            if let Err(source) = self
                .endpoint_registry
                .extend_endpoint_lease(&mut uow, self.endpoint_id, lease_until)
                .await
            {
                return Err(uow
                    .rollback_with_operation_error(ReadModelWatchEndpointError::from(source))
                    .await?);
            }
            uow.commit().await?;
        }
        Ok(())
    }

    async fn remove_closed_watch_senders(&self) -> Result<(), ReadModelWatchEndpointError> {
        let interval = self.config.endpoint_lease_refresh_interval().to_std();
        loop {
            tokio::select! {
                biased;
                _ = self.stop_token.cancelled() => return Ok(()),
                _ = tokio::time::sleep(interval) => {}
            }
            self.state
                .lock()
                .map_err(|_| ReadModelWatchEndpointError::StatePoisoned)?
                .watch_senders
                .retain(|_, sender| !sender.is_closed());
        }
    }
}

impl<F, ER, L, S> ReadModelWatchEndpoint for DefaultReadModelWatchEndpoint<F, ER, L, S>
where
    F: UnitOfWorkFactory,
    ER: ReadModelWatchEndpointRegistry<Uow = F::Uow>,
    L: ReadModelWatchLookup<Uow = F::Uow>,
    S: Subscriber<
            ReadModelWatchNotificationEnvelope,
            Selector = ReadModelWatchNotificationSelector,
        >,
{
    type Receiver = DefaultReadModelWatchReceiver;

    fn is_stop_requested(&self) -> bool {
        self.stop_token.is_cancelled()
    }

    fn request_graceful_stop(&self) {
        self.stop_token.cancel();
    }

    fn endpoint_id(&self) -> ReadModelWatchEndpointId {
        self.endpoint_id
    }

    async fn register_watch(
        &self,
        watch_id: ReadModelWatchId,
    ) -> Result<Self::Receiver, ReadModelWatchEndpointError> {
        let mut status = self
            .state
            .lock()
            .map_err(|_| ReadModelWatchEndpointError::StatePoisoned)?
            .status
            .subscribe();
        status
            .wait_for(|value| {
                matches!(
                    value,
                    DefaultReadModelWatchEndpointStatus::Running
                        | DefaultReadModelWatchEndpointStatus::Closed
                )
            })
            .await
            .map_err(|_| ReadModelWatchEndpointError::Closed)?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| ReadModelWatchEndpointError::StatePoisoned)?;
        if *state.status.borrow() != DefaultReadModelWatchEndpointStatus::Running {
            return Err(ReadModelWatchEndpointError::Closed);
        }
        if state.watch_senders.contains_key(&watch_id) {
            return Err(ReadModelWatchEndpointError::WatchAlreadyRegistered);
        }
        let (sender, receiver) = watch::channel(());
        state.watch_senders.insert(watch_id, sender);
        Ok(DefaultReadModelWatchReceiver::new(receiver))
    }

    async fn remove_watch(
        &self,
        watch_id: ReadModelWatchId,
    ) -> Result<(), ReadModelWatchEndpointError> {
        self.state
            .lock()
            .map_err(|_| ReadModelWatchEndpointError::StatePoisoned)?
            .watch_senders
            .remove(&watch_id);
        Ok(())
    }

    async fn run_forever(&self) -> Result<(), ReadModelWatchEndpointError> {
        {
            let state = self
                .state
                .lock()
                .map_err(|_| ReadModelWatchEndpointError::StatePoisoned)?;
            if *state.status.borrow() != DefaultReadModelWatchEndpointStatus::Idle {
                return Err(ReadModelWatchEndpointError::AlreadyStarted);
            }
            state
                .status
                .send_replace(DefaultReadModelWatchEndpointStatus::Starting);
        }
        let _guard = DefaultReadModelWatchEndpointRunGuard { state: &self.state };
        let consumer_group = ConsumerGroup::new(format!("read_model_watch_{}", self.endpoint_id))?;
        let selector = ReadModelWatchNotificationSelector::new(self.endpoint_id);
        let consumer = self
            .subscriber
            .subscribe(&consumer_group, Subscription::One(&selector))
            .await?;
        let mut uow = self.unit_of_work_factory.begin().await?;
        let lease_until =
            ReadModelWatchEndpointLeaseExpiresAt::from_now(self.config.endpoint_lease_duration());
        if let Err(source) = self
            .endpoint_registry
            .register_endpoint(&mut uow, self.endpoint_id, lease_until)
            .await
        {
            return Err(uow
                .rollback_with_operation_error(ReadModelWatchEndpointError::from(source))
                .await?);
        }
        uow.commit().await?;
        self.state
            .lock()
            .map_err(|_| ReadModelWatchEndpointError::StatePoisoned)?
            .status
            .send_replace(DefaultReadModelWatchEndpointStatus::Running);
        tokio::try_join!(
            self.receive_notifications(consumer),
            self.refresh_endpoint_lease(),
            self.remove_closed_watch_senders()
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        ReadModelWatchEndpointPage, ReadModelWatchEndpointPageSize,
        ReadModelWatchEndpointRegistryError, ReadModelWatchLookupError, ReadModelWatchNotification,
        ReadModelWatchPage, ReadModelWatchPageSize, ReadModelWatchReceiver, ReadModelWatchSignal,
    };
    use super::*;
    use crate::{
        messaging::{ConsumerError, DeliveryError, SubscriberError},
        read_model::{ReadModelChangeId, ReadModelId, ReadModelRef, ReadModelTypeOwned},
        unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError},
    };
    use chrono::Duration;
    use std::{
        collections::BTreeSet,
        future::pending,
        io,
        num::NonZeroU32,
        sync::{
            Arc,
            atomic::{AtomicBool, AtomicUsize, Ordering},
        },
        thread,
    };
    use tokio::sync::mpsc;

    #[derive(Default)]
    struct Backend {
        registered: AtomicBool,
        subscribed: AtomicBool,
        fail_lookup: AtomicBool,
        fail_endpoint_refresh: AtomicBool,
        block_subscription: AtomicBool,
        block_lookup: AtomicBool,
        lookup_started: AtomicBool,
        lookup_release: tokio::sync::Notify,
        watches: Mutex<BTreeSet<ReadModelWatchId>>,
        endpoint_refreshes: AtomicUsize,
        commits: AtomicUsize,
        rollbacks: AtomicUsize,
        acked: AtomicUsize,
        nacked: AtomicUsize,
    }

    #[derive(Clone)]
    struct Store(Arc<Backend>);

    struct TestUow(Arc<Backend>, bool, bool);

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            self.0.commits.fetch_add(1, Ordering::SeqCst);
            if self.1 {
                return Err(UnitOfWorkError::CommitFailed(Box::new(io::Error::other(
                    "commit failed",
                ))));
            }
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            self.0.rollbacks.fetch_add(1, Ordering::SeqCst);
            if self.2 {
                return Err(UnitOfWorkError::RollbackFailed(Box::new(io::Error::other(
                    "rollback failed",
                ))));
            }
            Ok(())
        }
    }

    impl UnitOfWorkFactory for Store {
        type Uow = TestUow;

        async fn begin(&self) -> Result<TestUow, UnitOfWorkFactoryError> {
            Ok(TestUow(self.0.clone(), false, false))
        }
    }

    impl ReadModelWatchEndpointRegistry for Store {
        type Uow = TestUow;

        async fn register_endpoint(
            &self,
            _: &mut TestUow,
            _: ReadModelWatchEndpointId,
            _: ReadModelWatchEndpointLeaseExpiresAt,
        ) -> Result<(), ReadModelWatchEndpointRegistryError> {
            assert!(self.0.subscribed.load(Ordering::SeqCst));
            self.0.registered.store(true, Ordering::SeqCst);
            Ok(())
        }

        async fn extend_endpoint_lease(
            &self,
            _: &mut TestUow,
            _: ReadModelWatchEndpointId,
            _: ReadModelWatchEndpointLeaseExpiresAt,
        ) -> Result<(), ReadModelWatchEndpointRegistryError> {
            self.0.endpoint_refreshes.fetch_add(1, Ordering::SeqCst);
            if self.0.fail_endpoint_refresh.load(Ordering::SeqCst) {
                return Err(ReadModelWatchEndpointRegistryError::Backend(Box::new(
                    io::Error::other("endpoint expired"),
                )));
            }
            Ok(())
        }

        async fn remove_endpoint(
            &self,
            _: &mut TestUow,
            _: ReadModelWatchEndpointId,
        ) -> Result<(), ReadModelWatchEndpointRegistryError> {
            Ok(())
        }
    }

    impl ReadModelWatchLookup for Store {
        type Uow = TestUow;

        async fn find_endpoint_ids_by_target(
            &self,
            _: &mut TestUow,
            _: &ReadModelRef,
            _: Option<ReadModelWatchEndpointId>,
            _: ReadModelWatchEndpointPageSize,
        ) -> Result<ReadModelWatchEndpointPage, ReadModelWatchLookupError> {
            unreachable!()
        }

        async fn find_watch_ids_by_target(
            &self,
            _: &mut TestUow,
            _: ReadModelWatchEndpointId,
            _: &ReadModelRef,
            cursor: Option<ReadModelWatchId>,
            limit: ReadModelWatchPageSize,
        ) -> Result<ReadModelWatchPage, ReadModelWatchLookupError> {
            self.0.lookup_started.store(true, Ordering::SeqCst);
            if self.0.block_lookup.load(Ordering::SeqCst) {
                self.0.lookup_release.notified().await;
            }
            if self.0.fail_lookup.load(Ordering::SeqCst) {
                return Err(ReadModelWatchLookupError::Backend(Box::new(
                    io::Error::other("lookup failed"),
                )));
            }
            let mut ids: Vec<_> = self
                .0
                .watches
                .lock()
                .unwrap()
                .iter()
                .copied()
                .filter(|id| cursor.is_none_or(|after| *id > after))
                .take(limit.as_usize() + 1)
                .collect();
            let next_cursor = if ids.len() > limit.as_usize() {
                ids.truncate(limit.as_usize());
                ids.last().copied()
            } else {
                None
            };
            Ok(ReadModelWatchPage::new(ids, next_cursor))
        }
    }

    struct TestSubscriber {
        backend: Arc<Backend>,
        receiver: Mutex<Option<mpsc::Receiver<ReadModelWatchNotificationEnvelope>>>,
    }

    struct TestConsumer {
        backend: Arc<Backend>,
        receiver: mpsc::Receiver<ReadModelWatchNotificationEnvelope>,
    }

    struct TestDelivery {
        backend: Arc<Backend>,
        envelope: ReadModelWatchNotificationEnvelope,
    }

    impl Subscriber<ReadModelWatchNotificationEnvelope> for TestSubscriber {
        type Consumer = TestConsumer;
        type Selector = ReadModelWatchNotificationSelector;

        async fn subscribe(
            &self,
            group: &ConsumerGroup,
            subscription: Subscription<'_, Self::Selector>,
        ) -> Result<TestConsumer, SubscriberError> {
            let Subscription::One(selector) = subscription else {
                panic!("expected endpoint selector")
            };
            assert!(group.value().ends_with(&selector.endpoint_id.to_string()));
            self.backend.subscribed.store(true, Ordering::SeqCst);
            if self.backend.block_subscription.load(Ordering::SeqCst) {
                pending::<()>().await;
            }
            Ok(TestConsumer {
                backend: self.backend.clone(),
                receiver: self.receiver.lock().unwrap().take().unwrap(),
            })
        }
    }

    impl Consumer<ReadModelWatchNotificationEnvelope> for TestConsumer {
        type Delivery = TestDelivery;

        async fn next(&mut self) -> Result<TestDelivery, ConsumerError> {
            let envelope = self
                .receiver
                .recv()
                .await
                .ok_or_else(|| ConsumerError::Next(Box::new(io::Error::other("closed"))))?;
            Ok(TestDelivery {
                backend: self.backend.clone(),
                envelope,
            })
        }
    }

    impl Delivery<ReadModelWatchNotificationEnvelope> for TestDelivery {
        fn message(&self) -> &ReadModelWatchNotificationEnvelope {
            &self.envelope
        }

        async fn ack(&mut self) -> Result<(), DeliveryError> {
            self.backend.acked.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        async fn nack(&mut self) -> Result<(), DeliveryError> {
            self.backend.nacked.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    type TestEndpoint = DefaultReadModelWatchEndpoint<Store, Store, Store, TestSubscriber>;

    fn setup(
        config: ReadModelWatchEndpointConfig,
    ) -> (
        Arc<TestEndpoint>,
        Arc<Backend>,
        mpsc::Sender<ReadModelWatchNotificationEnvelope>,
    ) {
        let backend = Arc::new(Backend::default());
        let store = Store(backend.clone());
        let (sender, receiver) = mpsc::channel(10);
        let subscriber = TestSubscriber {
            backend: backend.clone(),
            receiver: Mutex::new(Some(receiver)),
        };
        let endpoint = DefaultReadModelWatchEndpoint::new(
            store.clone(),
            store.clone(),
            store,
            subscriber,
            config,
        );
        (Arc::new(endpoint), backend, sender)
    }

    fn notification(endpoint_id: ReadModelWatchEndpointId) -> ReadModelWatchNotificationEnvelope {
        ReadModelWatchNotificationEnvelope::new(
            ReadModelChangeId::new(),
            ReadModelWatchNotification::new(
                endpoint_id,
                ReadModelRef {
                    read_model_type: ReadModelTypeOwned::try_from("user").unwrap(),
                    id: ReadModelId::from("one"),
                },
            ),
        )
    }

    async fn wait_for_count(counter: &AtomicUsize, minimum: usize) {
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            while counter.load(Ordering::SeqCst) < minimum {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn graceful_stop_interrupts_idle_receive_and_lease_sleep() {
        let (endpoint, backend, _sender) = setup(Default::default());
        let running_endpoint = endpoint.clone();
        let task = tokio::spawn(async move { running_endpoint.run_forever().await });
        let mut receiver = endpoint
            .register_watch(ReadModelWatchId::new())
            .await
            .unwrap();
        tokio::task::yield_now().await;
        endpoint.request_graceful_stop();
        tokio::time::timeout(std::time::Duration::from_secs(1), task)
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(
            receiver.receive().await.unwrap(),
            ReadModelWatchSignal::Closed
        );
        assert_eq!(backend.endpoint_refreshes.load(Ordering::SeqCst), 0);
        assert_eq!(backend.acked.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn graceful_stop_finishes_current_delivery_and_closes_receivers() {
        let config = ReadModelWatchEndpointConfig::new(
            Default::default(),
            Duration::milliseconds(10).try_into().unwrap(),
            ReadModelWatchEndpointConfig::default().watch_page_size(),
        )
        .unwrap();
        let (endpoint, backend, sender) = setup(config);
        assert!(!endpoint.is_stop_requested());
        let running_endpoint = endpoint.clone();
        let task = tokio::spawn(async move { running_endpoint.run_forever().await });
        let mut receiver = endpoint
            .register_watch(ReadModelWatchId::new())
            .await
            .unwrap();
        backend.block_lookup.store(true, Ordering::SeqCst);
        sender
            .send(notification(endpoint.endpoint_id()))
            .await
            .unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            while !backend.lookup_started.load(Ordering::SeqCst) {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        endpoint.request_graceful_stop();
        endpoint.request_graceful_stop();
        assert!(endpoint.is_stop_requested());
        assert!(!task.is_finished());
        assert_eq!(backend.acked.load(Ordering::SeqCst), 0);
        backend.lookup_release.notify_one();
        tokio::time::timeout(std::time::Duration::from_secs(2), task)
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(backend.acked.load(Ordering::SeqCst), 1);
        assert_eq!(
            receiver.receive().await.unwrap(),
            ReadModelWatchSignal::Closed
        );
        assert!(matches!(
            endpoint.register_watch(ReadModelWatchId::new()).await,
            Err(ReadModelWatchEndpointError::Closed)
        ));
    }

    #[tokio::test]
    async fn stop_requested_before_start_skips_processing_loops() {
        let (endpoint, backend, _sender) = setup(Default::default());
        endpoint.request_graceful_stop();
        tokio::time::timeout(std::time::Duration::from_secs(2), endpoint.run_forever())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(backend.endpoint_refreshes.load(Ordering::SeqCst), 0);
        assert_eq!(backend.acked.load(Ordering::SeqCst), 0);
        assert!(matches!(
            endpoint.register_watch(ReadModelWatchId::new()).await,
            Err(ReadModelWatchEndpointError::Closed)
        ));
    }

    #[tokio::test]
    async fn notifications_are_paginated_and_cancellation_closes_receivers() {
        let config = ReadModelWatchEndpointConfig::new(
            Default::default(),
            Default::default(),
            ReadModelWatchPageSize::new(NonZeroU32::MIN),
        )
        .unwrap();
        let (endpoint, backend, sender) = setup(config);
        let running_endpoint = endpoint.clone();
        let task = tokio::spawn(async move { running_endpoint.run_forever().await });
        let first_id = ReadModelWatchId::new();
        let second_id = ReadModelWatchId::new();
        backend
            .watches
            .lock()
            .unwrap()
            .extend([first_id, second_id]);
        let mut first = endpoint.register_watch(first_id).await.unwrap();
        let mut second = endpoint.register_watch(second_id).await.unwrap();
        assert!(matches!(
            endpoint.run_forever().await,
            Err(ReadModelWatchEndpointError::AlreadyStarted)
        ));
        sender
            .send(notification(ReadModelWatchEndpointId::new()))
            .await
            .unwrap();
        wait_for_count(&backend.acked, 1).await;
        sender
            .send(notification(endpoint.endpoint_id()))
            .await
            .unwrap();
        wait_for_count(&backend.acked, 2).await;
        assert_eq!(
            first.receive().await.unwrap(),
            ReadModelWatchSignal::Changed
        );
        assert_eq!(
            second.receive().await.unwrap(),
            ReadModelWatchSignal::Changed
        );
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
        assert_eq!(first.receive().await.unwrap(), ReadModelWatchSignal::Closed);
        assert_eq!(
            second.receive().await.unwrap(),
            ReadModelWatchSignal::Closed
        );
        assert!(matches!(
            endpoint.register_watch(ReadModelWatchId::new()).await,
            Err(ReadModelWatchEndpointError::Closed)
        ));
    }

    #[tokio::test]
    async fn dropped_receivers_only_remove_local_senders() {
        let config = ReadModelWatchEndpointConfig::new(
            Default::default(),
            Duration::milliseconds(10).try_into().unwrap(),
            ReadModelWatchEndpointConfig::default().watch_page_size(),
        )
        .unwrap();
        let (endpoint, backend, _sender) = setup(config);
        let running_endpoint = endpoint.clone();
        let task = tokio::spawn(async move { running_endpoint.run_forever().await });
        let watch_id = ReadModelWatchId::new();
        let receiver = endpoint.register_watch(watch_id).await.unwrap();
        backend.watches.lock().unwrap().insert(watch_id);
        drop(receiver);
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            loop {
                if !endpoint
                    .state
                    .lock()
                    .unwrap()
                    .watch_senders
                    .contains_key(&watch_id)
                {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        assert!(backend.watches.lock().unwrap().contains(&watch_id));
        assert!(backend.endpoint_refreshes.load(Ordering::SeqCst) > 0);
        task.abort();
        let _ = task.await;
    }

    #[tokio::test]
    async fn removal_only_closes_local_receiver() {
        let (endpoint, backend, _sender) = setup(Default::default());
        let running_endpoint = endpoint.clone();
        let task = tokio::spawn(async move { running_endpoint.run_forever().await });
        let watch_id = ReadModelWatchId::new();
        let mut receiver = endpoint.register_watch(watch_id).await.unwrap();
        assert!(matches!(
            endpoint.register_watch(watch_id).await,
            Err(ReadModelWatchEndpointError::WatchAlreadyRegistered)
        ));
        endpoint.remove_watch(watch_id).await.unwrap();
        assert_eq!(
            receiver.receive().await.unwrap(),
            ReadModelWatchSignal::Closed
        );
        assert_eq!(backend.rollbacks.load(Ordering::SeqCst), 0);
        assert_eq!(backend.commits.load(Ordering::SeqCst), 1);
        assert!(backend.watches.lock().unwrap().is_empty());
        endpoint.remove_watch(watch_id).await.unwrap();
        endpoint.remove_watch(watch_id).await.unwrap();
        task.abort();
        let _ = task.await;
    }

    #[tokio::test]
    async fn lookup_failure_nacks_and_closes_the_endpoint() {
        let (endpoint, backend, sender) = setup(Default::default());
        let running_endpoint = endpoint.clone();
        let task = tokio::spawn(async move { running_endpoint.run_forever().await });
        let mut receiver = endpoint
            .register_watch(ReadModelWatchId::new())
            .await
            .unwrap();
        backend.fail_lookup.store(true, Ordering::SeqCst);
        sender
            .send(notification(endpoint.endpoint_id()))
            .await
            .unwrap();
        assert!(task.await.unwrap().is_err());
        assert_eq!(backend.nacked.load(Ordering::SeqCst), 1);
        assert_eq!(backend.acked.load(Ordering::SeqCst), 0);
        assert_eq!(backend.rollbacks.load(Ordering::SeqCst), 1);
        assert_eq!(
            receiver.receive().await.unwrap(),
            ReadModelWatchSignal::Closed
        );
    }

    #[tokio::test]
    async fn cancellation_during_subscription_ends_waiting_registrations() {
        let (endpoint, backend, _sender) = setup(Default::default());
        backend.block_subscription.store(true, Ordering::SeqCst);
        let running_endpoint = endpoint.clone();
        let task = tokio::spawn(async move { running_endpoint.run_forever().await });
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            while !backend.subscribed.load(Ordering::SeqCst) {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        let registering_endpoint = endpoint.clone();
        let registration = tokio::spawn(async move {
            registering_endpoint
                .register_watch(ReadModelWatchId::new())
                .await
        });
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
        assert!(matches!(
            registration.await.unwrap(),
            Err(ReadModelWatchEndpointError::Closed)
        ));
        assert!(!backend.registered.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn stalled_notification_lookup_does_not_block_lease_refresh() {
        let config = ReadModelWatchEndpointConfig::new(
            Default::default(),
            Duration::milliseconds(10).try_into().unwrap(),
            ReadModelWatchEndpointConfig::default().watch_page_size(),
        )
        .unwrap();
        let (endpoint, backend, sender) = setup(config);
        let running_endpoint = endpoint.clone();
        let task = tokio::spawn(async move { running_endpoint.run_forever().await });
        let mut receiver = endpoint
            .register_watch(ReadModelWatchId::new())
            .await
            .unwrap();
        backend.block_lookup.store(true, Ordering::SeqCst);
        sender
            .send(notification(endpoint.endpoint_id()))
            .await
            .unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            while !backend.lookup_started.load(Ordering::SeqCst) {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        let refreshes = backend.endpoint_refreshes.load(Ordering::SeqCst);
        wait_for_count(&backend.endpoint_refreshes, refreshes + 1).await;
        assert_eq!(backend.acked.load(Ordering::SeqCst), 0);
        task.abort();
        let _ = task.await;
        assert_eq!(
            receiver.receive().await.unwrap(),
            ReadModelWatchSignal::Closed
        );
    }
    #[tokio::test]
    async fn endpoint_lease_failure_still_closes_all_receivers() {
        let config = ReadModelWatchEndpointConfig::new(
            Default::default(),
            Duration::milliseconds(10).try_into().unwrap(),
            ReadModelWatchEndpointConfig::default().watch_page_size(),
        )
        .unwrap();
        let (endpoint, backend, _sender) = setup(config);
        let running_endpoint = endpoint.clone();
        let task = tokio::spawn(async move { running_endpoint.run_forever().await });
        let mut first = endpoint
            .register_watch(ReadModelWatchId::new())
            .await
            .unwrap();
        let mut second = endpoint
            .register_watch(ReadModelWatchId::new())
            .await
            .unwrap();
        backend.fail_endpoint_refresh.store(true, Ordering::SeqCst);
        assert!(
            tokio::time::timeout(std::time::Duration::from_secs(2), task)
                .await
                .unwrap()
                .unwrap()
                .is_err()
        );
        assert_eq!(first.receive().await.unwrap(), ReadModelWatchSignal::Closed);
        assert_eq!(
            second.receive().await.unwrap(),
            ReadModelWatchSignal::Closed
        );
    }
    #[tokio::test]
    async fn poisoned_state_is_reported_by_normal_operations() {
        let (endpoint, _backend, _sender) = setup(Default::default());
        let poisoning_endpoint = endpoint.clone();
        assert!(
            thread::spawn(move || {
                let _state = poisoning_endpoint.state.lock().unwrap();
                panic!("poison endpoint state");
            })
            .join()
            .is_err()
        );
        assert!(matches!(
            endpoint.run_forever().await,
            Err(ReadModelWatchEndpointError::StatePoisoned)
        ));
        assert!(matches!(
            endpoint.register_watch(ReadModelWatchId::new()).await,
            Err(ReadModelWatchEndpointError::StatePoisoned)
        ));
        assert!(matches!(
            endpoint.remove_watch(ReadModelWatchId::new()).await,
            Err(ReadModelWatchEndpointError::StatePoisoned)
        ));
    }

    #[tokio::test]
    async fn shutdown_closes_receivers_even_when_state_is_poisoned() {
        let (endpoint, _backend, _sender) = setup(Default::default());
        let running_endpoint = endpoint.clone();
        let task = tokio::spawn(async move { running_endpoint.run_forever().await });
        let watch_id = ReadModelWatchId::new();
        let mut receiver = endpoint.register_watch(watch_id).await.unwrap();
        let poisoning_endpoint = endpoint.clone();
        assert!(
            thread::spawn(move || {
                let _state = poisoning_endpoint.state.lock().unwrap();
                panic!("poison endpoint state");
            })
            .join()
            .is_err()
        );
        assert!(matches!(
            endpoint.remove_watch(watch_id).await,
            Err(ReadModelWatchEndpointError::StatePoisoned)
        ));
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
        assert_eq!(
            receiver.receive().await.unwrap(),
            ReadModelWatchSignal::Closed
        );
    }
}
