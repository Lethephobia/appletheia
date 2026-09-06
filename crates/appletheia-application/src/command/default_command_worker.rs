use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use crate::Retryability;
use crate::command::{
    Command, CommandDispatcher, CommandEnvelope, CommandEnvelopeError,
    CommandExecutionFailureMarkResult, CommandExecutionLeaseAcquisitionResult,
    CommandExecutionLeaseReleaseResult, CommandExecutionStore, CommandFailureEnvelope,
    CommandHandler, CommandSelector, CommandTerminalReason, CommandWorker, CommandWorkerConfig,
    DefaultCommandWorkerDependencies,
};
use crate::messaging::Subscription;
use crate::outbox::command_failure::CommandFailureOutboxEnqueuer;
use crate::request_context::{ActorRef, Principal, RequestContext};
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};
use crate::{Consumer, ConsumerGroup, Delivery, Subscriber};

use super::CommandWorkerError;

pub struct DefaultCommandWorker<D, S, ES, FE, U> {
    dispatcher: D,
    subscriber: S,
    execution_store: ES,
    failure_outbox_enqueuer: FE,
    uow_factory: U,
    config: CommandWorkerConfig,
    stop_requested: AtomicBool,
}

impl<D, S, ES, FE, U> DefaultCommandWorker<D, S, ES, FE, U> {
    pub fn new(
        dependencies: DefaultCommandWorkerDependencies<D, S, ES, FE, U>,
        config: CommandWorkerConfig,
    ) -> Self {
        Self {
            dispatcher: dependencies.dispatcher,
            subscriber: dependencies.subscriber,
            execution_store: dependencies.execution_store,
            failure_outbox_enqueuer: dependencies.failure_outbox_enqueuer,
            uow_factory: dependencies.uow_factory,
            config,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<D, S, ES, FE, U> CommandWorker for DefaultCommandWorker<D, S, ES, FE, U>
where
    D: CommandDispatcher,
    S: Subscriber<CommandEnvelope, Selector = CommandSelector>,
    S::Consumer: Consumer<CommandEnvelope>,
    <S::Consumer as Consumer<CommandEnvelope>>::Delivery: Delivery<CommandEnvelope>,
    ES: CommandExecutionStore<Uow = D::Uow>,
    FE: CommandFailureOutboxEnqueuer<Uow = D::Uow>,
    U: UnitOfWorkFactory<Uow = D::Uow>,
{
    type Uow = D::Uow;

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_forever<H>(&self, handler: &H) -> Result<(), CommandWorkerError>
    where
        H: CommandHandler<Uow = Self::Uow>,
    {
        let consumer_group = ConsumerGroup::from(H::Command::NAME);
        let selectors = [CommandSelector::new(H::Command::NAME)];

        let mut consumer = self
            .subscriber
            .subscribe(&consumer_group, Subscription::AnyOf(&selectors))
            .await?;

        while !self.is_stop_requested() {
            let mut delivery = consumer.next().await?;

            let decoded_command = match delivery.message().try_into_command::<H::Command>() {
                Ok(command) => Some(command),
                Err(CommandEnvelopeError::CommandNameMismatch { .. }) => {
                    delivery.ack().await?;
                    None
                }
                Err(error) => {
                    delivery.nack().await?;
                    return Err(error.into());
                }
            };

            if let Some(command) = decoded_command {
                let envelope = delivery.message();
                let mut lease_uow = self.uow_factory.begin().await?;
                let lease_acquisition = self
                    .execution_store
                    .acquire_lease(&mut lease_uow, envelope, self.config.lease_duration)
                    .await;
                let lease_acquisition_result = match lease_acquisition {
                    Ok(result) => {
                        lease_uow.commit().await?;
                        result
                    }
                    Err(error) => {
                        let rolled_back_error =
                            lease_uow.rollback_with_operation_error(error).await?;
                        return Err(rolled_back_error.into());
                    }
                };
                let attempt_count = match lease_acquisition_result {
                    CommandExecutionLeaseAcquisitionResult::Acquired { attempt_count } => {
                        attempt_count
                    }
                    CommandExecutionLeaseAcquisitionResult::Succeeded
                    | CommandExecutionLeaseAcquisitionResult::Failed => {
                        delivery.ack().await?;
                        continue;
                    }
                    CommandExecutionLeaseAcquisitionResult::InProgress => {
                        delivery.nack().await?;
                        continue;
                    }
                };
                let request_context = RequestContext {
                    correlation_id: envelope.correlation_id,
                    message_id: envelope.message_id,
                    actor: ActorRef::System,
                    principal: Principal::System,
                };

                let result = self
                    .dispatcher
                    .dispatch(handler, &request_context, command, envelope.options.clone())
                    .await;

                match result {
                    Ok(_) => {
                        let mut success_uow = self.uow_factory.begin().await?;
                        let mark_succeeded_result = self
                            .execution_store
                            .mark_succeeded(&mut success_uow, envelope.message_id)
                            .await;
                        match mark_succeeded_result {
                            Ok(()) => success_uow.commit().await?,
                            Err(store_error) => {
                                let rolled_back_error = success_uow
                                    .rollback_with_operation_error(store_error)
                                    .await?;
                                return Err(rolled_back_error.into());
                            }
                        }
                        delivery.ack().await?;
                    }
                    Err(error) => {
                        let retryable = error.is_retryable();
                        let has_attempts_remaining = attempt_count.value()
                            < self.config.retry_options.max_attempts.value().get();
                        if retryable && has_attempts_remaining {
                            let mut release_uow = self.uow_factory.begin().await?;
                            let release_attempt = self
                                .execution_store
                                .release_lease(&mut release_uow, envelope.message_id, attempt_count)
                                .await;
                            let release_result = match release_attempt {
                                Ok(result) => result,
                                Err(store_error) => {
                                    let rolled_back_error = release_uow
                                        .rollback_with_operation_error(store_error)
                                        .await?;
                                    return Err(rolled_back_error.into());
                                }
                            };
                            release_uow.commit().await?;
                            match release_result {
                                CommandExecutionLeaseReleaseResult::Released => {
                                    delivery.nack().await?;
                                }
                                CommandExecutionLeaseReleaseResult::Stale => {
                                    delivery.ack().await?;
                                }
                            }
                        } else {
                            let reason = if retryable {
                                CommandTerminalReason::RetryExhausted
                            } else {
                                CommandTerminalReason::NonRetryable
                            };
                            let mut failure_uow = self.uow_factory.begin().await?;
                            let mark_failed_attempt = self
                                .execution_store
                                .mark_failed(&mut failure_uow, envelope.message_id, attempt_count)
                                .await;
                            let mark_failed_result = match mark_failed_attempt {
                                Ok(result) => result,
                                Err(store_error) => {
                                    let rolled_back_error = failure_uow
                                        .rollback_with_operation_error(store_error)
                                        .await?;
                                    return Err(rolled_back_error.into());
                                }
                            };
                            match mark_failed_result {
                                CommandExecutionFailureMarkResult::Marked { failed_at } => {
                                    if let Some(origin) = envelope.saga_origin.clone() {
                                        let notification = CommandFailureEnvelope::new(
                                            envelope,
                                            origin,
                                            reason,
                                            attempt_count,
                                            failed_at,
                                        );
                                        self.failure_outbox_enqueuer
                                            .enqueue_command_failure(
                                                &mut failure_uow,
                                                &notification,
                                            )
                                            .await?;
                                    }
                                    failure_uow.commit().await?;
                                    delivery.ack().await?;
                                }
                                CommandExecutionFailureMarkResult::Stale => {
                                    failure_uow.commit().await?;
                                    delivery.ack().await?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use super::{CommandWorkerError, DefaultCommandWorker};
    use crate::Retryability;
    use crate::authorization::AuthorizationPlan;
    use crate::command::{
        Command, CommandAttemptCount, CommandDispatchResult, CommandDispatcher,
        CommandDispatcherError, CommandEnvelope, CommandExecutionFailureMarkResult,
        CommandExecutionLeaseAcquisitionResult, CommandExecutionLeaseDuration,
        CommandExecutionLeaseReleaseResult, CommandExecutionMaxAttempts,
        CommandExecutionRetryOptions, CommandExecutionStore, CommandExecutionStoreError,
        CommandFailedAt, CommandHandler, CommandName, CommandOptions, CommandOutput,
        CommandReplayOutput, CommandWorker, CommandWorkerConfig, DefaultCommandWorkerDependencies,
    };
    use crate::messaging::{
        Consumer, ConsumerError, ConsumerGroup, Delivery, Subscriber, SubscriberError, Subscription,
    };
    use crate::outbox::command_failure::{
        CommandFailureOutboxEnqueueError, CommandFailureOutboxEnqueuer,
    };
    use crate::request_context::{CausationId, CorrelationId, MessageId, RequestContext};
    use crate::unit_of_work::{
        UnitOfWork, UnitOfWorkError, UnitOfWorkFactory, UnitOfWorkFactoryError,
    };

    struct TestUow {
        events: Arc<Mutex<Vec<&'static str>>>,
    }

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            self.events.lock().unwrap().push("committed");
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            self.events.lock().unwrap().push("rolled_back");
            Ok(())
        }
    }

    struct TestUowFactory {
        events: Arc<Mutex<Vec<&'static str>>>,
    }

    impl UnitOfWorkFactory for TestUowFactory {
        type Uow = TestUow;

        async fn begin(&self) -> Result<Self::Uow, UnitOfWorkFactoryError> {
            self.events.lock().unwrap().push("begun");
            Ok(TestUow {
                events: Arc::clone(&self.events),
            })
        }
    }

    struct TestExecutionStore {
        events: Arc<Mutex<Vec<&'static str>>>,
    }

    impl CommandExecutionStore for TestExecutionStore {
        type Uow = TestUow;

        async fn acquire_lease(
            &self,
            _uow: &mut Self::Uow,
            _command: &CommandEnvelope,
            _lease_duration: CommandExecutionLeaseDuration,
        ) -> Result<CommandExecutionLeaseAcquisitionResult, CommandExecutionStoreError> {
            self.events.lock().unwrap().push("lease_acquired");
            Ok(CommandExecutionLeaseAcquisitionResult::Acquired {
                attempt_count: CommandAttemptCount::first(),
            })
        }

        async fn mark_succeeded(
            &self,
            _uow: &mut Self::Uow,
            _command_message_id: MessageId,
        ) -> Result<(), CommandExecutionStoreError> {
            self.events.lock().unwrap().push("succeeded");
            Ok(())
        }

        async fn mark_failed(
            &self,
            _uow: &mut Self::Uow,
            _command_message_id: MessageId,
            _attempt_count: CommandAttemptCount,
        ) -> Result<CommandExecutionFailureMarkResult, CommandExecutionStoreError> {
            self.events.lock().unwrap().push("failed");
            Ok(CommandExecutionFailureMarkResult::Marked {
                failed_at: CommandFailedAt::now(),
            })
        }

        async fn release_lease(
            &self,
            _uow: &mut Self::Uow,
            _command_message_id: MessageId,
            _attempt_count: CommandAttemptCount,
        ) -> Result<CommandExecutionLeaseReleaseResult, CommandExecutionStoreError> {
            self.events.lock().unwrap().push("lease_released");
            Ok(CommandExecutionLeaseReleaseResult::Released)
        }
    }

    struct TestFailureOutboxEnqueuer;

    impl CommandFailureOutboxEnqueuer for TestFailureOutboxEnqueuer {
        type Uow = TestUow;

        async fn enqueue_command_failure(
            &self,
            _uow: &mut Self::Uow,
            _failure: &crate::command::CommandFailureEnvelope,
        ) -> Result<(), CommandFailureOutboxEnqueueError> {
            Ok(())
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct TestCommand {}

    impl Command for TestCommand {
        const NAME: CommandName = CommandName::new("test");
    }

    #[derive(Debug, thiserror::Error)]
    #[error("test handler error")]
    struct TestHandlerError {
        retryable: bool,
    }

    impl Retryability for TestHandlerError {
        fn is_retryable(&self) -> bool {
            self.retryable
        }
    }

    struct TestHandler {
        failure_retryability: Option<bool>,
    }

    #[derive(Deserialize, Serialize)]
    struct TestOutput;

    impl CommandOutput for TestOutput {
        type ReplayOutput = Self;

        fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
            CommandReplayOutput::Borrowed(self)
        }
    }

    impl CommandHandler for TestHandler {
        type Command = TestCommand;
        type Output = TestOutput;
        type Error = TestHandlerError;
        type Uow = TestUow;

        fn authorization_plan(
            &self,
            _command: &Self::Command,
        ) -> Result<AuthorizationPlan, Self::Error> {
            match self.failure_retryability {
                Some(retryable) => Err(TestHandlerError { retryable }),
                None => Ok(AuthorizationPlan::default()),
            }
        }

        async fn handle(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            _command: &Self::Command,
        ) -> Result<Self::Output, Self::Error> {
            Ok(TestOutput)
        }
    }

    struct TestDispatcher {
        events: Arc<Mutex<Vec<&'static str>>>,
    }

    impl CommandDispatcher for TestDispatcher {
        type Uow = TestUow;

        async fn dispatch<H>(
            &self,
            handler: &H,
            _request_context: &RequestContext,
            command: H::Command,
            _options: CommandOptions,
        ) -> Result<
            CommandDispatchResult<H::Output, <H::Output as CommandOutput>::ReplayOutput>,
            CommandDispatcherError<H::Error>,
        >
        where
            H: CommandHandler<Uow = Self::Uow>,
            H::Command: Command,
        {
            self.events.lock().unwrap().push("dispatched");
            match handler.authorization_plan(&command) {
                Ok(_) => {
                    let mut uow = TestUow {
                        events: Arc::clone(&self.events),
                    };
                    handler
                        .handle(&mut uow, _request_context, &command)
                        .await
                        .map(CommandDispatchResult::Executed)
                        .map_err(CommandDispatcherError::Handler)
                }
                Err(error) => Err(CommandDispatcherError::Handler(error)),
            }
        }
    }

    #[derive(Default)]
    struct DeliveryState {
        acknowledgements: AtomicUsize,
        negative_acknowledgements: AtomicUsize,
        consumer_groups: Mutex<Vec<String>>,
        events: Arc<Mutex<Vec<&'static str>>>,
    }

    struct TestDelivery {
        envelope: CommandEnvelope,
        state: Arc<DeliveryState>,
    }

    impl Delivery<CommandEnvelope> for TestDelivery {
        fn message(&self) -> &CommandEnvelope {
            &self.envelope
        }

        async fn ack(&mut self) -> Result<(), ConsumerError> {
            self.state.events.lock().unwrap().push("acked");
            self.state.acknowledgements.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        async fn nack(&mut self) -> Result<(), ConsumerError> {
            self.state.events.lock().unwrap().push("nacked");
            self.state
                .negative_acknowledgements
                .fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    struct TestConsumer {
        delivery: Option<TestDelivery>,
    }

    impl Consumer<CommandEnvelope> for TestConsumer {
        type Delivery = TestDelivery;

        async fn next(&mut self) -> Result<Self::Delivery, ConsumerError> {
            self.delivery.take().ok_or_else(|| {
                ConsumerError::Next(Box::new(std::io::Error::other("test consumer exhausted")))
            })
        }
    }

    fn assert_consumer_exhausted(error: CommandWorkerError) {
        assert!(matches!(
            error,
            CommandWorkerError::Consumer(ConsumerError::Next(_))
        ));
    }

    struct TestSubscriber {
        envelope: CommandEnvelope,
        state: Arc<DeliveryState>,
    }

    impl Subscriber<CommandEnvelope> for TestSubscriber {
        type Consumer = TestConsumer;
        type Selector = crate::command::CommandSelector;

        async fn subscribe(
            &self,
            consumer_group: &ConsumerGroup,
            _subscription: Subscription<'_, Self::Selector>,
        ) -> Result<Self::Consumer, SubscriberError> {
            self.state
                .consumer_groups
                .lock()
                .unwrap()
                .push(consumer_group.value().to_owned());
            Ok(TestConsumer {
                delivery: Some(TestDelivery {
                    envelope: self.envelope.clone(),
                    state: Arc::clone(&self.state),
                }),
            })
        }
    }

    fn build_worker(
        max_attempts: NonZeroU32,
        state: Arc<DeliveryState>,
    ) -> DefaultCommandWorker<
        TestDispatcher,
        TestSubscriber,
        TestExecutionStore,
        TestFailureOutboxEnqueuer,
        TestUowFactory,
    > {
        let causation_message_id = MessageId::new();
        let envelope = CommandEnvelope::new(
            &TestCommand {},
            CorrelationId::from(Uuid::now_v7()),
            CausationId::from(causation_message_id),
            CommandOptions::default(),
        )
        .expect("command envelope should be valid");
        let events = Arc::clone(&state.events);
        DefaultCommandWorker::new(
            DefaultCommandWorkerDependencies {
                dispatcher: TestDispatcher {
                    events: Arc::clone(&events),
                },
                subscriber: TestSubscriber { envelope, state },
                execution_store: TestExecutionStore {
                    events: Arc::clone(&events),
                },
                failure_outbox_enqueuer: TestFailureOutboxEnqueuer,
                uow_factory: TestUowFactory { events },
            },
            CommandWorkerConfig {
                lease_duration: CommandExecutionLeaseDuration::default(),
                retry_options: CommandExecutionRetryOptions {
                    max_attempts: CommandExecutionMaxAttempts::new(max_attempts),
                },
            },
        )
    }

    #[tokio::test]
    async fn run_forever_nacks_retryable_dispatch_failure_and_continues() {
        let state = Arc::new(DeliveryState::default());
        let command_worker = build_worker(NonZeroU32::new(2).unwrap(), Arc::clone(&state));
        let handler = TestHandler {
            failure_retryability: Some(true),
        };

        let error = command_worker
            .run_forever(&handler)
            .await
            .expect_err("worker should continue until the test consumer is exhausted");
        assert_consumer_exhausted(error);

        assert_eq!(state.acknowledgements.load(Ordering::SeqCst), 0);
        assert_eq!(state.negative_acknowledgements.load(Ordering::SeqCst), 1);
        assert_eq!(
            *state.events.lock().unwrap(),
            [
                "begun",
                "lease_acquired",
                "committed",
                "dispatched",
                "begun",
                "lease_released",
                "committed",
                "nacked",
            ]
        );
    }

    #[tokio::test]
    async fn run_forever_acks_non_retryable_dispatch_failure_and_continues() {
        let state = Arc::new(DeliveryState::default());
        let command_worker = build_worker(NonZeroU32::new(2).unwrap(), Arc::clone(&state));
        let handler = TestHandler {
            failure_retryability: Some(false),
        };

        let error = command_worker
            .run_forever(&handler)
            .await
            .expect_err("worker should continue until the test consumer is exhausted");
        assert_consumer_exhausted(error);

        assert_eq!(state.acknowledgements.load(Ordering::SeqCst), 1);
        assert_eq!(state.negative_acknowledgements.load(Ordering::SeqCst), 0);
        assert_eq!(
            *state.events.lock().unwrap(),
            [
                "begun",
                "lease_acquired",
                "committed",
                "dispatched",
                "begun",
                "failed",
                "committed",
                "acked",
            ]
        );
    }

    #[tokio::test]
    async fn run_forever_marks_success_in_a_separate_uow_before_acknowledging() {
        let state = Arc::new(DeliveryState::default());
        let command_worker = build_worker(NonZeroU32::new(2).unwrap(), Arc::clone(&state));
        let handler = TestHandler {
            failure_retryability: None,
        };

        let error = command_worker
            .run_forever(&handler)
            .await
            .expect_err("test consumer should be exhausted after successful delivery");
        assert_consumer_exhausted(error);

        assert_eq!(
            *state.events.lock().unwrap(),
            [
                "begun",
                "lease_acquired",
                "committed",
                "dispatched",
                "begun",
                "succeeded",
                "committed",
                "acked",
            ]
        );
        assert_eq!(state.acknowledgements.load(Ordering::SeqCst), 1);
        assert_eq!(state.negative_acknowledgements.load(Ordering::SeqCst), 0);
        assert_eq!(*state.consumer_groups.lock().unwrap(), ["test"]);
    }

    #[tokio::test]
    async fn run_forever_marks_exhausted_retryable_failure_and_continues() {
        let state = Arc::new(DeliveryState::default());
        let command_worker = build_worker(NonZeroU32::new(1).unwrap(), Arc::clone(&state));
        let handler = TestHandler {
            failure_retryability: Some(true),
        };

        let error = command_worker
            .run_forever(&handler)
            .await
            .expect_err("worker should continue until the test consumer is exhausted");
        assert_consumer_exhausted(error);

        assert_eq!(state.acknowledgements.load(Ordering::SeqCst), 1);
        assert_eq!(state.negative_acknowledgements.load(Ordering::SeqCst), 0);
        assert_eq!(
            *state.events.lock().unwrap(),
            [
                "begun",
                "lease_acquired",
                "committed",
                "dispatched",
                "begun",
                "failed",
                "committed",
                "acked",
            ]
        );
    }
}
