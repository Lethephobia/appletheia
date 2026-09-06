use crate::command::CommandFailureEnvelope;

use crate::event::EventEnvelope;
use crate::outbox::command::CommandOutboxEnqueuer;
use crate::request_context::MessageId;
use crate::unit_of_work::UnitOfWork;
use crate::unit_of_work::UnitOfWorkFactory;

use super::SagaInstance;
use super::{
    EnqueuedCommandCount, Saga, SagaCommandFailureRunReport, SagaEventRunReport, SagaInstanceStore,
    SagaNameOwned, SagaProcessedCommandFailureStore, SagaProcessedEventStore, SagaRunner,
    SagaRunnerError, SagaSpec, SagaStatus,
};

pub struct DefaultSagaRunner<S, P, F, Q, U> {
    saga_instance_store: S,
    processed_event_store: P,
    processed_command_failure_store: F,
    command_outbox_enqueuer: Q,
    uow_factory: U,
}

impl<S, P, F, Q, U> DefaultSagaRunner<S, P, F, Q, U> {
    pub fn new(
        saga_instance_store: S,
        processed_event_store: P,
        processed_command_failure_store: F,
        command_outbox_enqueuer: Q,
        uow_factory: U,
    ) -> Self {
        Self {
            saga_instance_store,
            processed_event_store,
            processed_command_failure_store,
            command_outbox_enqueuer,
            uow_factory,
        }
    }
}

impl<S, P, F, Q, U> DefaultSagaRunner<S, P, F, Q, U>
where
    S: SagaInstanceStore,
    P: SagaProcessedEventStore<Uow = S::Uow>,
    F: SagaProcessedCommandFailureStore<Uow = S::Uow>,
    Q: CommandOutboxEnqueuer<Uow = S::Uow>,
    U: UnitOfWorkFactory<Uow = S::Uow>,
{
    async fn handle_event_inner<SG: Saga>(
        &self,
        uow: &mut S::Uow,
        saga: &SG,
        event: &EventEnvelope,
    ) -> Result<SagaEventRunReport, SagaRunnerError> {
        let descriptor = <SG::Spec as SagaSpec>::DESCRIPTOR;
        if !descriptor.subscription.matches(event) {
            return Ok(SagaEventRunReport::NotSubscribed);
        }

        let saga_name = SagaNameOwned::from(descriptor.name);
        let correlation_id = event.correlation_id;

        let (mut instance, causative_step) = if descriptor.start_events.matches(event) {
            let instance = self
                .saga_instance_store
                .find_by_correlation_id::<<SG::Spec as SagaSpec>::State, SG::Step>(
                    uow,
                    saga_name.clone(),
                    correlation_id,
                )
                .await?
                .unwrap_or_else(|| {
                    SagaInstance::new(saga_name.clone(), correlation_id, event.event_id)
                });
            (instance, None)
        } else {
            let command_message_id = MessageId::from(event.causation_id.value());
            let Some(instance) = self
                .saga_instance_store
                .find_by_dispatched_command_message_id::<<SG::Spec as SagaSpec>::State, SG::Step>(
                    uow,
                    saga_name.clone(),
                    command_message_id,
                )
                .await?
            else {
                return Ok(SagaEventRunReport::InstanceNotFound);
            };
            let Some(dispatched_command) = instance
                .dispatched_commands
                .iter()
                .find(|command| command.message_id == command_message_id)
            else {
                return Ok(SagaEventRunReport::CommandNotOwned);
            };
            let causative_step = dispatched_command.step;
            (instance, Some(causative_step))
        };

        if instance.is_terminal() {
            let report = if instance.is_succeeded() {
                SagaEventRunReport::SkippedSucceeded
            } else {
                SagaEventRunReport::SkippedFailed
            };
            return Ok(report);
        }

        let inserted = self
            .processed_event_store
            .mark_processed(uow, saga_name.clone(), correlation_id, event.event_id)
            .await?;
        if !inserted {
            return Ok(SagaEventRunReport::AlreadyProcessed);
        }

        saga.on_event(&mut instance, event, causative_step)
            .map_err(|source| SagaRunnerError::Definition(Box::new(source)))?;

        self.saga_instance_store.save(uow, &instance).await?;

        let commands = instance.uncommitted_commands().to_vec();
        let enqueued_command_count = EnqueuedCommandCount::from_usize_saturating(commands.len());
        if !commands.is_empty() {
            self.command_outbox_enqueuer
                .enqueue_commands(uow, &commands)
                .await?;
        }

        let report = match &instance.status {
            SagaStatus::InProgress => SagaEventRunReport::InProgress {
                enqueued_command_count,
            },
            SagaStatus::Succeeded => SagaEventRunReport::Succeeded,
            SagaStatus::Failed => SagaEventRunReport::Failed,
        };

        Ok(report)
    }

    async fn handle_command_failure_inner<SG: Saga>(
        &self,
        uow: &mut S::Uow,
        saga: &SG,
        failure: &CommandFailureEnvelope,
    ) -> Result<SagaCommandFailureRunReport, SagaRunnerError> {
        let descriptor = <SG::Spec as SagaSpec>::DESCRIPTOR;
        if failure.origin.saga_name.value() != descriptor.name.value() {
            return Ok(SagaCommandFailureRunReport::NotSubscribed);
        }

        let saga_name = SagaNameOwned::from(descriptor.name);
        let Some(mut instance) = self
            .saga_instance_store
            .find_by_dispatched_command_message_id::<<SG::Spec as SagaSpec>::State, SG::Step>(
                uow,
                saga_name.clone(),
                failure.command_message_id,
            )
            .await?
        else {
            return Ok(SagaCommandFailureRunReport::InstanceNotFound);
        };
        let Some(dispatched_command) = instance
            .dispatched_commands
            .iter()
            .find(|command| command.message_id == failure.command_message_id)
        else {
            return Ok(SagaCommandFailureRunReport::CommandNotOwned);
        };
        let causative_step = dispatched_command.step;

        if instance.is_terminal() {
            let report = if instance.is_succeeded() {
                SagaCommandFailureRunReport::SkippedSucceeded
            } else {
                SagaCommandFailureRunReport::SkippedFailed
            };
            return Ok(report);
        }

        let inserted = self
            .processed_command_failure_store
            .mark_processed(
                uow,
                instance.saga_instance_id,
                failure.failure_id,
                failure.command_message_id,
            )
            .await?;
        if !inserted {
            return Ok(SagaCommandFailureRunReport::AlreadyProcessed);
        }

        saga.on_command_failed(&mut instance, failure, causative_step)
            .map_err(|source| SagaRunnerError::Definition(Box::new(source)))?;
        self.saga_instance_store.save(uow, &instance).await?;
        let commands = instance.uncommitted_commands().to_vec();
        let enqueued_command_count = EnqueuedCommandCount::from_usize_saturating(commands.len());
        if !commands.is_empty() {
            self.command_outbox_enqueuer
                .enqueue_commands(uow, &commands)
                .await?;
        }
        let report = match instance.status {
            SagaStatus::InProgress => SagaCommandFailureRunReport::InProgress {
                enqueued_command_count,
            },
            SagaStatus::Succeeded => SagaCommandFailureRunReport::Succeeded,
            SagaStatus::Failed => SagaCommandFailureRunReport::Failed,
        };
        Ok(report)
    }
}

impl<S, P, F, Q, U> SagaRunner for DefaultSagaRunner<S, P, F, Q, U>
where
    S: SagaInstanceStore,
    P: SagaProcessedEventStore<Uow = S::Uow>,
    F: SagaProcessedCommandFailureStore<Uow = S::Uow>,
    Q: CommandOutboxEnqueuer<Uow = S::Uow>,
    U: UnitOfWorkFactory<Uow = S::Uow>,
{
    async fn handle_event<SG: Saga>(
        &self,
        saga: &SG,
        event: &EventEnvelope,
    ) -> Result<SagaEventRunReport, SagaRunnerError> {
        let mut uow = self.uow_factory.begin().await?;

        let result = self.handle_event_inner(&mut uow, saga, event).await;
        match result {
            Ok(report) => {
                uow.commit().await?;
                Ok(report)
            }
            Err(error) => Err(uow.rollback_with_operation_error(error).await?),
        }
    }

    async fn handle_command_failure<SG: Saga>(
        &self,
        saga: &SG,
        failure: &CommandFailureEnvelope,
    ) -> Result<SagaCommandFailureRunReport, SagaRunnerError> {
        let mut uow = self.uow_factory.begin().await?;
        let result = self
            .handle_command_failure_inner(&mut uow, saga, failure)
            .await;
        match result {
            Ok(report) => {
                uow.commit().await?;
                Ok(report)
            }
            Err(error) => Err(uow.rollback_with_operation_error(error).await?),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    };

    use appletheia_domain::{AggregateVersion, EventId};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use thiserror::Error;
    use uuid::Uuid;

    use super::DefaultSagaRunner;
    use crate::command::{
        CommandAttemptCount, CommandEnvelope, CommandFailedAt, CommandFailureEnvelope,
        CommandFailureId, CommandNameOwned, CommandTerminalReason,
    };
    use crate::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSelector,
        EventSequence, SerializedEventPayload,
    };
    use crate::messaging::Subscription;
    use crate::outbox::command::{CommandOutboxEnqueueError, CommandOutboxEnqueuer};
    use crate::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use crate::saga::{
        Saga, SagaCommandFailureRunReport, SagaCommandOrigin, SagaDescriptor,
        SagaDispatchedCommand, SagaEventRunReport, SagaInstance, SagaInstanceId, SagaInstanceStore,
        SagaInstanceStoreError, SagaName, SagaNameOwned, SagaProcessedCommandFailureStore,
        SagaProcessedCommandFailureStoreError, SagaProcessedEventStore,
        SagaProcessedEventStoreError, SagaRunner, SagaSpec, SagaStartEvents, SagaState, SagaStatus,
        SagaStep, SerializedSagaStep,
    };
    use crate::unit_of_work::{
        UnitOfWork, UnitOfWorkError, UnitOfWorkFactory, UnitOfWorkFactoryError,
    };

    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    struct TestUowFactory;

    impl UnitOfWorkFactory for TestUowFactory {
        type Uow = TestUow;

        async fn begin(&self) -> Result<Self::Uow, UnitOfWorkFactoryError> {
            Ok(TestUow)
        }
    }

    struct TerminalSagaInstanceStore;

    impl SagaInstanceStore for TerminalSagaInstanceStore {
        type Uow = TestUow;

        async fn find_by_correlation_id<S: SagaState, T: SagaStep>(
            &self,
            _uow: &mut Self::Uow,
            saga_name: SagaNameOwned,
            correlation_id: CorrelationId,
        ) -> Result<Option<SagaInstance<S, T>>, SagaInstanceStoreError> {
            let mut instance = SagaInstance::new(saga_name, correlation_id, EventId::new());
            instance.status = SagaStatus::Succeeded;
            Ok(Some(instance))
        }

        async fn find_by_dispatched_command_message_id<S: SagaState, T: SagaStep>(
            &self,
            _uow: &mut Self::Uow,
            _saga_name: SagaNameOwned,
            _dispatched_command_message_id: MessageId,
        ) -> Result<Option<SagaInstance<S, T>>, SagaInstanceStoreError> {
            Ok(None)
        }

        async fn save<S: SagaState, T: SagaStep>(
            &self,
            _uow: &mut Self::Uow,
            _instance: &SagaInstance<S, T>,
        ) -> Result<(), SagaInstanceStoreError> {
            Ok(())
        }
    }

    struct CountingProcessedEventStore {
        calls: Arc<AtomicUsize>,
    }

    impl SagaProcessedEventStore for CountingProcessedEventStore {
        type Uow = TestUow;

        async fn mark_processed(
            &self,
            _uow: &mut Self::Uow,
            _saga_name: SagaNameOwned,
            _correlation_id: CorrelationId,
            _event_id: EventId,
        ) -> Result<bool, SagaProcessedEventStoreError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(true)
        }
    }

    struct TestProcessedCommandFailureStore;

    impl SagaProcessedCommandFailureStore for TestProcessedCommandFailureStore {
        type Uow = TestUow;

        async fn mark_processed(
            &self,
            _uow: &mut Self::Uow,
            _saga_instance_id: SagaInstanceId,
            _command_failure_id: CommandFailureId,
            _command_message_id: MessageId,
        ) -> Result<bool, SagaProcessedCommandFailureStoreError> {
            Ok(true)
        }
    }

    struct TestCommandOutboxEnqueuer;

    impl CommandOutboxEnqueuer for TestCommandOutboxEnqueuer {
        type Uow = TestUow;

        async fn enqueue_commands(
            &self,
            _uow: &mut Self::Uow,
            _commands: &[CommandEnvelope],
        ) -> Result<(), CommandOutboxEnqueueError> {
            Ok(())
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    struct TestSagaState;

    impl SagaState for TestSagaState {}

    struct TestSagaSpec;

    impl SagaSpec for TestSagaSpec {
        type State = TestSagaState;

        const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
            SagaName::new("test_saga"),
            SagaStartEvents::new(&[EventSelector::from_parts(
                appletheia_domain::AggregateType::new("user"),
                appletheia_domain::EventName::new("user_registered"),
            )]),
            Subscription::All,
        );
    }

    struct TestSaga;

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum TestSagaStep {
        FollowUp,
    }

    impl SagaStep for TestSagaStep {}

    #[derive(Debug, Error)]
    #[error("unexpected on_event call")]
    struct TestSagaError;

    impl Saga for TestSaga {
        type Spec = TestSagaSpec;
        type Step = TestSagaStep;
        type Error = TestSagaError;

        fn on_event(
            &self,
            _instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
            _event: &EventEnvelope,
            _causative_step: Option<Self::Step>,
        ) -> Result<(), Self::Error> {
            panic!("on_event must not be called for terminal saga instances");
        }
    }

    struct SucceedWithoutStateSaga;

    impl Saga for SucceedWithoutStateSaga {
        type Spec = TestSagaSpec;
        type Step = TestSagaStep;
        type Error = TestSagaError;

        fn on_event(
            &self,
            instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
            _event: &EventEnvelope,
            causative_step: Option<Self::Step>,
        ) -> Result<(), Self::Error> {
            assert_eq!(causative_step, None);
            instance.succeed();
            Ok(())
        }
    }

    struct AssertFollowUpSaga;

    impl Saga for AssertFollowUpSaga {
        type Spec = TestSagaSpec;
        type Step = TestSagaStep;
        type Error = TestSagaError;

        fn on_event(
            &self,
            instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State, Self::Step>,
            _event: &EventEnvelope,
            causative_step: Option<Self::Step>,
        ) -> Result<(), Self::Error> {
            assert_eq!(causative_step, Some(TestSagaStep::FollowUp));
            instance.succeed();
            Ok(())
        }
    }

    fn test_event() -> EventEnvelope {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let message_id = MessageId::from(Uuid::now_v7());

        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("event sequence"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::try_from("user").expect("aggregate type"),
            aggregate_id: AggregateIdValue::from(Uuid::now_v7()),
            aggregate_version: AggregateVersion::try_from(1).expect("aggregate version"),
            event_name: EventNameOwned::try_from("user_registered").expect("event name"),
            payload: SerializedEventPayload::try_from(json!({ "event": "user_registered" }))
                .expect("payload"),
            occurred_at: appletheia_domain::EventOccurredAt::now(),
            correlation_id,
            causation_id: CausationId::from(message_id),
            context: RequestContext::new(correlation_id, message_id, Principal::System)
                .expect("request context should be valid"),
        }
    }

    #[tokio::test]
    async fn handle_event_skips_terminal_saga_before_marking_processed() {
        let calls = Arc::new(AtomicUsize::new(0));
        let runner = DefaultSagaRunner::new(
            TerminalSagaInstanceStore,
            CountingProcessedEventStore {
                calls: Arc::clone(&calls),
            },
            TestProcessedCommandFailureStore,
            TestCommandOutboxEnqueuer,
            TestUowFactory,
        );

        let report = runner
            .handle_event(&TestSaga, &test_event())
            .await
            .expect("terminal saga should be skipped");

        assert_eq!(report, SagaEventRunReport::SkippedSucceeded);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn handle_event_allows_terminal_saga_without_state() {
        let runner = DefaultSagaRunner::new(
            TerminalSagaInstanceStoreForNewInstance,
            CountingProcessedEventStore {
                calls: Arc::new(AtomicUsize::new(0)),
            },
            TestProcessedCommandFailureStore,
            TestCommandOutboxEnqueuer,
            TestUowFactory,
        );

        let report = runner
            .handle_event(&SucceedWithoutStateSaga, &test_event())
            .await
            .expect("terminal saga without state should be accepted");

        assert_eq!(report, SagaEventRunReport::Succeeded);
    }

    #[tokio::test]
    async fn handle_event_resolves_step_from_causative_dispatched_command() {
        let command_message_id = MessageId::new();
        let mut event = test_event();
        event.event_name = EventNameOwned::try_from("user_updated").expect("event name");
        event.causation_id = CausationId::from(command_message_id);
        let runner = DefaultSagaRunner::new(
            DispatchedCommandSagaInstanceStore {
                command_message_id,
                correlation_id: event.correlation_id,
            },
            CountingProcessedEventStore {
                calls: Arc::new(AtomicUsize::new(0)),
            },
            TestProcessedCommandFailureStore,
            TestCommandOutboxEnqueuer,
            TestUowFactory,
        );
        let report = runner
            .handle_event(&AssertFollowUpSaga, &event)
            .await
            .expect("causative saga step should resolve");

        assert_eq!(report, SagaEventRunReport::Succeeded);
    }

    #[tokio::test]
    async fn handle_command_failure_resolves_persisted_step_and_fails_by_default() {
        let command_message_id = MessageId::new();
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let saved_status = Arc::new(Mutex::new(None));
        let runner = DefaultSagaRunner::new(
            CommandFailureSagaInstanceStore {
                command_message_id,
                correlation_id,
                saved_status: Arc::clone(&saved_status),
            },
            CountingProcessedEventStore {
                calls: Arc::new(AtomicUsize::new(0)),
            },
            TestProcessedCommandFailureStore,
            TestCommandOutboxEnqueuer,
            TestUowFactory,
        );
        let failure = CommandFailureEnvelope {
            failure_id: CommandFailureId::new(),
            command_message_id,
            command_name: CommandNameOwned::new("follow_up".to_owned()).expect("command name"),
            origin: SagaCommandOrigin {
                saga_name: SagaNameOwned::from(TestSagaSpec::DESCRIPTOR.name),
                saga_instance_id: SagaInstanceId::new(),
                step: SerializedSagaStep::new(TestSagaStep::FollowUp).expect("serialize step"),
            },
            terminal_reason: CommandTerminalReason::NonRetryable,
            attempt_count: CommandAttemptCount::first(),
            correlation_id,
            causation_id: CausationId::from(command_message_id),
            failed_at: CommandFailedAt::now(),
        };

        let report = runner
            .handle_command_failure(&TestSaga, &failure)
            .await
            .expect("command failure should be handled");

        assert_eq!(report, SagaCommandFailureRunReport::Failed);
        assert_eq!(
            *saved_status.lock().expect("saved status lock"),
            Some(SagaStatus::Failed)
        );
    }

    struct DispatchedCommandSagaInstanceStore {
        command_message_id: MessageId,
        correlation_id: CorrelationId,
    }

    struct CommandFailureSagaInstanceStore {
        command_message_id: MessageId,
        correlation_id: CorrelationId,
        saved_status: Arc<Mutex<Option<SagaStatus>>>,
    }

    impl SagaInstanceStore for CommandFailureSagaInstanceStore {
        type Uow = TestUow;

        async fn find_by_correlation_id<S: SagaState, T: SagaStep>(
            &self,
            _uow: &mut Self::Uow,
            _saga_name: SagaNameOwned,
            _correlation_id: CorrelationId,
        ) -> Result<Option<SagaInstance<S, T>>, SagaInstanceStoreError> {
            Ok(None)
        }

        async fn find_by_dispatched_command_message_id<S: SagaState, T: SagaStep>(
            &self,
            _uow: &mut Self::Uow,
            saga_name: SagaNameOwned,
            dispatched_command_message_id: MessageId,
        ) -> Result<Option<SagaInstance<S, T>>, SagaInstanceStoreError> {
            assert_eq!(dispatched_command_message_id, self.command_message_id);
            let mut instance = SagaInstance::new(saga_name, self.correlation_id, EventId::new());
            let step = serde_json::from_value(json!("follow_up")).expect("saga step");
            instance.dispatched_commands.push(SagaDispatchedCommand {
                message_id: self.command_message_id,
                command_name: CommandNameOwned::new("follow_up".to_owned()).expect("command name"),
                step,
            });
            Ok(Some(instance))
        }

        async fn save<S: SagaState, T: SagaStep>(
            &self,
            _uow: &mut Self::Uow,
            instance: &SagaInstance<S, T>,
        ) -> Result<(), SagaInstanceStoreError> {
            *self.saved_status.lock().expect("saved status lock") = Some(instance.status.clone());
            Ok(())
        }
    }

    impl SagaInstanceStore for DispatchedCommandSagaInstanceStore {
        type Uow = TestUow;

        async fn find_by_correlation_id<S: SagaState, T: SagaStep>(
            &self,
            _uow: &mut Self::Uow,
            _saga_name: SagaNameOwned,
            _correlation_id: CorrelationId,
        ) -> Result<Option<SagaInstance<S, T>>, SagaInstanceStoreError> {
            Ok(None)
        }

        async fn find_by_dispatched_command_message_id<S: SagaState, T: SagaStep>(
            &self,
            _uow: &mut Self::Uow,
            saga_name: SagaNameOwned,
            dispatched_command_message_id: MessageId,
        ) -> Result<Option<SagaInstance<S, T>>, SagaInstanceStoreError> {
            assert_eq!(dispatched_command_message_id, self.command_message_id);
            let mut instance = SagaInstance::new(saga_name, self.correlation_id, EventId::new());
            let step = serde_json::from_value(json!("follow_up")).expect("saga step");
            instance.dispatched_commands.push(SagaDispatchedCommand {
                message_id: self.command_message_id,
                command_name: CommandNameOwned::new("follow_up".to_owned()).expect("command name"),
                step,
            });
            Ok(Some(instance))
        }

        async fn save<S: SagaState, T: SagaStep>(
            &self,
            _uow: &mut Self::Uow,
            _instance: &SagaInstance<S, T>,
        ) -> Result<(), SagaInstanceStoreError> {
            Ok(())
        }
    }

    struct TerminalSagaInstanceStoreForNewInstance;

    impl SagaInstanceStore for TerminalSagaInstanceStoreForNewInstance {
        type Uow = TestUow;

        async fn find_by_correlation_id<S: SagaState, T: SagaStep>(
            &self,
            _uow: &mut Self::Uow,
            saga_name: SagaNameOwned,
            correlation_id: CorrelationId,
        ) -> Result<Option<SagaInstance<S, T>>, SagaInstanceStoreError> {
            Ok(Some(SagaInstance::new(
                saga_name,
                correlation_id,
                EventId::new(),
            )))
        }

        async fn find_by_dispatched_command_message_id<S: SagaState, T: SagaStep>(
            &self,
            _uow: &mut Self::Uow,
            _saga_name: SagaNameOwned,
            _dispatched_command_message_id: MessageId,
        ) -> Result<Option<SagaInstance<S, T>>, SagaInstanceStoreError> {
            Ok(None)
        }

        async fn save<S: SagaState, T: SagaStep>(
            &self,
            _uow: &mut Self::Uow,
            _instance: &SagaInstance<S, T>,
        ) -> Result<(), SagaInstanceStoreError> {
            Ok(())
        }
    }
}
