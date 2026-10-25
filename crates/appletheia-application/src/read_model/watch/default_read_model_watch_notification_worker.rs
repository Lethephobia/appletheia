use tokio_util::sync::CancellationToken;

use crate::messaging::{Consumer, Delivery, PublishResult, Publisher, Subscriber, Subscription};
use crate::read_model::{ReadModelChangeEnvelope, ReadModelChangeSelector};
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{
    ReadModelWatchLookup, ReadModelWatchNotification, ReadModelWatchNotificationEnvelope,
    ReadModelWatchNotificationWorker, ReadModelWatchNotificationWorkerConfig,
    ReadModelWatchNotificationWorkerError,
};

pub struct DefaultReadModelWatchNotificationWorker<F, L, S, P>
where
    F: UnitOfWorkFactory,
    L: ReadModelWatchLookup<Uow = F::Uow>,
    S: Subscriber<ReadModelChangeEnvelope, Selector = ReadModelChangeSelector>,
    P: Publisher<Message = ReadModelWatchNotificationEnvelope>,
{
    unit_of_work_factory: F,
    watch_lookup: L,
    subscriber: S,
    publisher: P,
    config: ReadModelWatchNotificationWorkerConfig,
    stop_token: CancellationToken,
}

impl<F, L, S, P> DefaultReadModelWatchNotificationWorker<F, L, S, P>
where
    F: UnitOfWorkFactory,
    L: ReadModelWatchLookup<Uow = F::Uow>,
    S: Subscriber<ReadModelChangeEnvelope, Selector = ReadModelChangeSelector>,
    P: Publisher<Message = ReadModelWatchNotificationEnvelope>,
{
    pub fn new(
        unit_of_work_factory: F,
        watch_lookup: L,
        subscriber: S,
        publisher: P,
        config: ReadModelWatchNotificationWorkerConfig,
    ) -> Self {
        Self {
            unit_of_work_factory,
            watch_lookup,
            subscriber,
            publisher,
            config,
            stop_token: CancellationToken::new(),
        }
    }

    async fn notify_endpoints(
        &self,
        envelope: &ReadModelChangeEnvelope,
    ) -> Result<(), ReadModelWatchNotificationWorkerError> {
        let target = envelope.change.read_model_ref();
        let mut cursor = None;
        loop {
            let mut uow = self.unit_of_work_factory.begin().await?;
            let page = match self
                .watch_lookup
                .find_endpoint_ids_by_target(
                    &mut uow,
                    target,
                    cursor,
                    self.config.endpoint_page_size,
                )
                .await
            {
                Ok(page) => page,
                Err(source) => {
                    return Err(uow
                        .rollback_with_operation_error(
                            ReadModelWatchNotificationWorkerError::Lookup(source),
                        )
                        .await?);
                }
            };
            uow.commit().await?;
            let notifications: Vec<_> = page
                .endpoint_ids
                .into_iter()
                .map(|endpoint_id| {
                    ReadModelWatchNotificationEnvelope::new(
                        envelope.change_id,
                        ReadModelWatchNotification::new(endpoint_id, target.clone()),
                    )
                })
                .collect();
            if !notifications.is_empty() {
                let results = self.publisher.publish(&notifications).await?;
                let failures: Vec<_> = results
                    .into_iter()
                    .filter_map(|result| match result {
                        PublishResult::Failed { input_index, cause } => Some((input_index, cause)),
                        PublishResult::Success { .. } => None,
                    })
                    .collect();
                if !failures.is_empty() {
                    return Err(ReadModelWatchNotificationWorkerError::PublishFailed { failures });
                }
            }
            cursor = page.next_cursor;
            if cursor.is_none() {
                return Ok(());
            }
        }
    }
}

impl<F, L, S, P> ReadModelWatchNotificationWorker
    for DefaultReadModelWatchNotificationWorker<F, L, S, P>
where
    F: UnitOfWorkFactory,
    L: ReadModelWatchLookup<Uow = F::Uow>,
    S: Subscriber<ReadModelChangeEnvelope, Selector = ReadModelChangeSelector>,
    P: Publisher<Message = ReadModelWatchNotificationEnvelope>,
{
    type Uow = F::Uow;

    fn is_stop_requested(&self) -> bool {
        self.stop_token.is_cancelled()
    }

    fn request_graceful_stop(&self) {
        self.stop_token.cancel();
    }

    async fn run_forever(&self) -> Result<(), ReadModelWatchNotificationWorkerError> {
        let mut consumer = self
            .subscriber
            .subscribe(&self.config.consumer_group, Subscription::All)
            .await?;
        while !self.is_stop_requested() {
            let mut delivery = tokio::select! {
                biased;
                _ = self.stop_token.cancelled() => break,
                delivery = consumer.next() => delivery?,
            };
            match self.notify_endpoints(delivery.message()).await {
                Ok(()) => delivery.ack().await?,
                Err(error) => {
                    delivery.nack().await?;
                    return Err(error);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::VecDeque,
        io,
        num::NonZeroU32,
        sync::{Arc, Mutex},
        time::Duration,
    };

    use appletheia_domain::EventId;
    use tokio::sync::Notify;
    use tokio::time::timeout;

    use super::*;
    use crate::messaging::{
        ConsumerError, ConsumerGroup, DeliveryError, PublishDispatchError, PublisherError,
        SubscriberError,
    };
    use crate::read_model::watch::{
        ReadModelWatchEndpointId, ReadModelWatchEndpointPage, ReadModelWatchEndpointPageSize,
        ReadModelWatchId, ReadModelWatchLookupError, ReadModelWatchPage, ReadModelWatchPageSize,
    };
    use crate::read_model::{ReadModelChange, ReadModelId, ReadModelRef};
    use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

    #[derive(Default)]
    struct State {
        pages: VecDeque<ReadModelWatchEndpointPage>,
        cursors: Vec<Option<ReadModelWatchEndpointId>>,
        notifications: Vec<ReadModelWatchNotificationEnvelope>,
        operations: Vec<&'static str>,
        failure: Option<&'static str>,
    }

    #[derive(Clone)]
    struct Backend {
        state: Arc<Mutex<State>>,
        settled: Arc<Notify>,
        publishing: Arc<Notify>,
        resume: Arc<Notify>,
        block_publish: bool,
        envelope: ReadModelChangeEnvelope,
    }

    struct TestUow(Backend);

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            self.0.state.lock().unwrap().operations.push("commit");
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            self.0.state.lock().unwrap().operations.push("rollback");
            Ok(())
        }
    }

    impl UnitOfWorkFactory for Backend {
        type Uow = TestUow;

        async fn begin(&self) -> Result<TestUow, UnitOfWorkFactoryError> {
            Ok(TestUow(self.clone()))
        }
    }

    impl ReadModelWatchLookup for Backend {
        type Uow = TestUow;

        async fn find_endpoint_ids_by_target(
            &self,
            _: &mut TestUow,
            target: &ReadModelRef,
            cursor: Option<ReadModelWatchEndpointId>,
            _: ReadModelWatchEndpointPageSize,
        ) -> Result<ReadModelWatchEndpointPage, ReadModelWatchLookupError> {
            assert_eq!(target, self.envelope.change.read_model_ref());
            let mut state = self.state.lock().unwrap();
            state.operations.push("lookup");
            state.cursors.push(cursor);
            if state.failure == Some("lookup") {
                return Err(ReadModelWatchLookupError::Backend(Box::new(
                    io::Error::other("lookup failed"),
                )));
            }
            Ok(state.pages.pop_front().expect("expected page"))
        }

        async fn find_watch_ids_by_target(
            &self,
            _: &mut TestUow,
            _: ReadModelWatchEndpointId,
            _: &ReadModelRef,
            _: Option<ReadModelWatchId>,
            _: ReadModelWatchPageSize,
        ) -> Result<ReadModelWatchPage, ReadModelWatchLookupError> {
            panic!("watch lookup belongs to endpoint")
        }
    }

    impl Publisher for Backend {
        type Message = ReadModelWatchNotificationEnvelope;

        async fn publish<'a, I>(&self, messages: I) -> Result<Vec<PublishResult>, PublisherError>
        where
            I: IntoIterator<Item = &'a Self::Message>,
            Self::Message: 'a,
        {
            let notifications: Vec<_> = messages.into_iter().cloned().collect();
            if self.block_publish {
                self.publishing.notify_one();
                self.resume.notified().await;
            }
            let mut state = self.state.lock().unwrap();
            state.operations.push("publish");
            assert_eq!(state.operations[state.operations.len() - 2], "commit");
            state.notifications.extend(notifications.clone());
            if state.failure == Some("publisher") {
                return Err(PublisherError::Publish(Box::new(io::Error::other(
                    "publish failed",
                ))));
            }
            Ok(notifications
                .iter()
                .enumerate()
                .map(|(input_index, _)| {
                    if state.failure == Some("partial") && input_index == 1
                        || state.failure == Some("multiple")
                    {
                        PublishResult::Failed {
                            input_index,
                            cause: PublishDispatchError::Transient {
                                code: "unavailable".into(),
                                message: "retry".into(),
                            },
                        }
                    } else {
                        PublishResult::Success {
                            input_index,
                            transport_message_id: None,
                        }
                    }
                })
                .collect())
        }
    }

    struct TestConsumer {
        backend: Backend,
        received: bool,
    }

    struct TestDelivery(Backend);

    impl Subscriber<ReadModelChangeEnvelope> for Backend {
        type Consumer = TestConsumer;
        type Selector = ReadModelChangeSelector;

        async fn subscribe(
            &self,
            group: &ConsumerGroup,
            subscription: Subscription<'_, Self::Selector>,
        ) -> Result<TestConsumer, SubscriberError> {
            assert_eq!(group.value(), "watch_notification");
            assert!(matches!(subscription, Subscription::All));
            Ok(TestConsumer {
                backend: self.clone(),
                received: false,
            })
        }
    }

    impl Consumer<ReadModelChangeEnvelope> for TestConsumer {
        type Delivery = TestDelivery;

        async fn next(&mut self) -> Result<TestDelivery, ConsumerError> {
            if self.received {
                std::future::pending::<()>().await;
            }
            self.received = true;
            Ok(TestDelivery(self.backend.clone()))
        }
    }

    impl Delivery<ReadModelChangeEnvelope> for TestDelivery {
        fn message(&self) -> &ReadModelChangeEnvelope {
            &self.0.envelope
        }

        async fn ack(&mut self) -> Result<(), DeliveryError> {
            self.0.state.lock().unwrap().operations.push("ack");
            self.0.settled.notify_one();
            Ok(())
        }

        async fn nack(&mut self) -> Result<(), DeliveryError> {
            self.0.state.lock().unwrap().operations.push("nack");
            Ok(())
        }
    }

    fn backend(pages: Vec<ReadModelWatchEndpointPage>) -> Backend {
        Backend {
            state: Arc::new(Mutex::new(State {
                pages: pages.into(),
                ..State::default()
            })),
            settled: Arc::new(Notify::new()),
            publishing: Arc::new(Notify::new()),
            resume: Arc::new(Notify::new()),
            block_publish: false,
            envelope: ReadModelChangeEnvelope::new(
                EventId::new(),
                ReadModelChange::new(ReadModelRef {
                    read_model_type: "user".try_into().unwrap(),
                    id: ReadModelId::new("one".into()),
                }),
            ),
        }
    }

    fn worker(
        backend: &Backend,
    ) -> DefaultReadModelWatchNotificationWorker<Backend, Backend, Backend, Backend> {
        DefaultReadModelWatchNotificationWorker::new(
            backend.clone(),
            backend.clone(),
            backend.clone(),
            backend.clone(),
            ReadModelWatchNotificationWorkerConfig {
                consumer_group: ConsumerGroup::new("watch_notification".into()).unwrap(),
                endpoint_page_size: ReadModelWatchEndpointPageSize::new(
                    NonZeroU32::new(2).unwrap(),
                ),
            },
        )
    }

    #[tokio::test]
    async fn publishes_all_pages_before_acknowledging_and_interrupts_idle_wait() {
        let first = ReadModelWatchEndpointId::new();
        let second = ReadModelWatchEndpointId::new();
        let backend = backend(vec![
            ReadModelWatchEndpointPage::new(vec![first], Some(first)),
            ReadModelWatchEndpointPage::new(vec![second], None),
        ]);
        let worker = worker(&backend);
        let (result, ()) = timeout(Duration::from_secs(1), async {
            tokio::join!(worker.run_forever(), async {
                backend.settled.notified().await;
                worker.request_graceful_stop();
            })
        })
        .await
        .unwrap();
        result.unwrap();
        let state = backend.state.lock().unwrap();
        assert_eq!(
            state.operations,
            [
                "lookup", "commit", "publish", "lookup", "commit", "publish", "ack"
            ]
        );
        assert_eq!(state.cursors, [None, Some(first)]);
        assert_eq!(
            state
                .notifications
                .iter()
                .map(|message| message.notification.endpoint_id)
                .collect::<Vec<_>>(),
            [first, second]
        );
        for message in &state.notifications {
            assert_eq!(message.source_change_id, backend.envelope.change_id);
            assert_eq!(
                &message.notification.target,
                backend.envelope.change.read_model_ref()
            );
        }
    }

    #[tokio::test]
    async fn no_matching_endpoints_are_acknowledged_without_publishing() {
        let backend = backend(vec![ReadModelWatchEndpointPage::new(vec![], None)]);
        let worker = worker(&backend);
        let (result, ()) = tokio::join!(worker.run_forever(), async {
            backend.settled.notified().await;
            worker.request_graceful_stop();
        });
        result.unwrap();
        assert_eq!(
            backend.state.lock().unwrap().operations,
            ["lookup", "commit", "ack"]
        );
    }

    #[tokio::test]
    async fn failures_nack_instead_of_acknowledging() {
        for failure in ["lookup", "publisher", "partial", "multiple"] {
            let backend = backend(vec![ReadModelWatchEndpointPage::new(
                vec![
                    ReadModelWatchEndpointId::new(),
                    ReadModelWatchEndpointId::new(),
                ],
                None,
            )]);
            backend.state.lock().unwrap().failure = Some(failure);
            let worker = worker(&backend);
            let error = worker.run_forever().await.unwrap_err();
            if failure == "partial" || failure == "multiple" {
                let ReadModelWatchNotificationWorkerError::PublishFailed { failures } = error
                else {
                    panic!("expected publication failures");
                };
                let cause = PublishDispatchError::Transient {
                    code: "unavailable".into(),
                    message: "retry".into(),
                };
                let expected = if failure == "multiple" {
                    vec![(0, cause.clone()), (1, cause)]
                } else {
                    vec![(1, cause)]
                };
                assert_eq!(failures, expected);
            }
            let state = backend.state.lock().unwrap();
            assert_eq!(state.operations.last(), Some(&"nack"));
            assert!(!state.operations.contains(&"ack"));
            if failure == "lookup" {
                assert_eq!(state.operations, ["lookup", "rollback", "nack"]);
            }
        }
    }

    #[tokio::test]
    async fn stop_during_publication_finishes_delivery_before_exiting() {
        let mut backend = backend(vec![ReadModelWatchEndpointPage::new(
            vec![ReadModelWatchEndpointId::new()],
            None,
        )]);
        backend.block_publish = true;
        let worker = worker(&backend);
        let (result, ()) = timeout(Duration::from_secs(1), async {
            tokio::join!(worker.run_forever(), async {
                backend.publishing.notified().await;
                worker.request_graceful_stop();
                assert!(!backend.state.lock().unwrap().operations.contains(&"ack"));
                backend.resume.notify_one();
            })
        })
        .await
        .unwrap();
        result.unwrap();
        assert_eq!(
            backend.state.lock().unwrap().operations,
            ["lookup", "commit", "publish", "ack"]
        );
    }
}
