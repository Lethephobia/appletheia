use crate::event::EventEnvelope;
use crate::outbox::command::CommandOutboxEnqueuer;
use crate::unit_of_work::UnitOfWork;
use crate::unit_of_work::UnitOfWorkFactory;

use super::SagaInstance;
use super::{
    EnqueuedCommandCount, Saga, SagaNameOwned, SagaProcessedEventStore, SagaRunReport, SagaRunner,
    SagaRunnerError, SagaSpec, SagaStatus, SagaStore,
};

pub struct DefaultSagaRunner<S, P, Q, U> {
    saga_store: S,
    processed_event_store: P,
    command_outbox_enqueuer: Q,
    uow_factory: U,
}

impl<S, P, Q, U> DefaultSagaRunner<S, P, Q, U> {
    pub fn new(
        saga_store: S,
        processed_event_store: P,
        command_outbox_enqueuer: Q,
        uow_factory: U,
    ) -> Self {
        Self {
            saga_store,
            processed_event_store,
            command_outbox_enqueuer,
            uow_factory,
        }
    }
}

impl<S, P, Q, U> DefaultSagaRunner<S, P, Q, U>
where
    S: SagaStore,
    P: SagaProcessedEventStore<Uow = S::Uow>,
    Q: CommandOutboxEnqueuer<Uow = S::Uow>,
    U: UnitOfWorkFactory<Uow = S::Uow>,
{
    async fn handle_event_inner<SG: Saga>(
        &self,
        uow: &mut S::Uow,
        saga: &SG,
        event: &EventEnvelope,
    ) -> Result<(SagaInstance<<SG::Spec as SagaSpec>::State>, SagaRunReport), SagaRunnerError> {
        let descriptor = <SG::Spec as SagaSpec>::DESCRIPTOR;
        let saga_name = SagaNameOwned::from(descriptor.name);
        let correlation_id = event.correlation_id;

        let mut instance = self
            .saga_store
            .load::<<SG::Spec as SagaSpec>::State>(uow, saga_name.clone(), correlation_id)
            .await?;

        if instance.is_terminal() {
            let report = if instance.is_succeeded() {
                SagaRunReport::SkippedSucceeded
            } else {
                SagaRunReport::SkippedFailed
            };
            return Ok((instance, report));
        }

        let inserted = self
            .processed_event_store
            .mark_processed(uow, saga_name.clone(), correlation_id, event.event_id)
            .await?;
        if !inserted {
            return Ok((instance, SagaRunReport::AlreadyProcessed));
        }

        saga.on_event(&mut instance, event)
            .map_err(|source| SagaRunnerError::Definition(Box::new(source)))?;

        self.saga_store.save(uow, &instance).await?;

        let commands = instance.uncommitted_commands().to_vec();
        let enqueued_command_count = EnqueuedCommandCount::from_usize_saturating(commands.len());
        if !commands.is_empty() {
            self.command_outbox_enqueuer
                .enqueue_commands(uow, &commands)
                .await?;
        }

        let report = match &instance.status {
            SagaStatus::InProgress => SagaRunReport::InProgress {
                enqueued_command_count,
            },
            SagaStatus::Succeeded => SagaRunReport::Succeeded,
            SagaStatus::Failed => SagaRunReport::Failed,
        };

        Ok((instance, report))
    }
}

impl<S, P, Q, U> SagaRunner for DefaultSagaRunner<S, P, Q, U>
where
    S: SagaStore,
    P: SagaProcessedEventStore<Uow = S::Uow>,
    Q: CommandOutboxEnqueuer<Uow = S::Uow>,
    U: UnitOfWorkFactory<Uow = S::Uow>,
{
    async fn handle_event<SG: Saga>(
        &self,
        saga: &SG,
        event: &EventEnvelope,
    ) -> Result<SagaRunReport, SagaRunnerError> {
        let mut uow = self.uow_factory.begin().await?;

        let result = self.handle_event_inner(&mut uow, saga, event).await;
        match result {
            Ok((mut instance, report)) => {
                uow.commit().await?;
                instance.clear_uncommitted_commands();
                Ok(report)
            }
            Err(error) => Err(uow.rollback_with_operation_error(error).await?),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    use appletheia_domain::{AggregateVersion, EventId};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use thiserror::Error;
    use uuid::Uuid;

    use super::DefaultSagaRunner;
    use crate::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
        SerializedEventPayload,
    };
    use crate::messaging::Subscription;
    use crate::outbox::command::{
        CommandEnvelope, CommandOutboxEnqueueError, CommandOutboxEnqueuer,
    };
    use crate::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use crate::saga::{
        Saga, SagaDescriptor, SagaInstance, SagaName, SagaNameOwned, SagaProcessedEventStore,
        SagaProcessedEventStoreError, SagaRunReport, SagaRunner, SagaSpec, SagaState, SagaStatus,
        SagaStore, SagaStoreError,
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

    struct TerminalSagaStore;

    impl SagaStore for TerminalSagaStore {
        type Uow = TestUow;

        async fn load<S: SagaState>(
            &self,
            _uow: &mut Self::Uow,
            saga_name: SagaNameOwned,
            correlation_id: CorrelationId,
        ) -> Result<SagaInstance<S>, SagaStoreError> {
            let mut instance = SagaInstance::new(saga_name, correlation_id);
            instance.status = SagaStatus::Succeeded;
            Ok(instance)
        }

        async fn save<S: SagaState>(
            &self,
            _uow: &mut Self::Uow,
            _instance: &SagaInstance<S>,
        ) -> Result<(), SagaStoreError> {
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

        const DESCRIPTOR: SagaDescriptor =
            SagaDescriptor::new(SagaName::new("test_saga"), Subscription::All);
    }

    struct TestSaga;

    #[derive(Debug, Error)]
    #[error("unexpected on_event call")]
    struct TestSagaError;

    impl Saga for TestSaga {
        type Spec = TestSagaSpec;
        type Error = TestSagaError;

        fn on_event(
            &self,
            _instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
            _event: &EventEnvelope,
        ) -> Result<(), Self::Error> {
            panic!("on_event must not be called for terminal saga instances");
        }
    }

    struct SucceedWithoutStateSaga;

    impl Saga for SucceedWithoutStateSaga {
        type Spec = TestSagaSpec;
        type Error = TestSagaError;

        fn on_event(
            &self,
            instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
            _event: &EventEnvelope,
        ) -> Result<(), Self::Error> {
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
            TerminalSagaStore,
            CountingProcessedEventStore {
                calls: Arc::clone(&calls),
            },
            TestCommandOutboxEnqueuer,
            TestUowFactory,
        );

        let report = runner
            .handle_event(&TestSaga, &test_event())
            .await
            .expect("terminal saga should be skipped");

        assert_eq!(report, SagaRunReport::SkippedSucceeded);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn handle_event_allows_terminal_saga_without_state() {
        let runner = DefaultSagaRunner::new(
            TerminalSagaStoreForNewInstance,
            CountingProcessedEventStore {
                calls: Arc::new(AtomicUsize::new(0)),
            },
            TestCommandOutboxEnqueuer,
            TestUowFactory,
        );

        let report = runner
            .handle_event(&SucceedWithoutStateSaga, &test_event())
            .await
            .expect("terminal saga without state should be accepted");

        assert_eq!(report, SagaRunReport::Succeeded);
    }

    struct TerminalSagaStoreForNewInstance;

    impl SagaStore for TerminalSagaStoreForNewInstance {
        type Uow = TestUow;

        async fn load<S: SagaState>(
            &self,
            _uow: &mut Self::Uow,
            saga_name: SagaNameOwned,
            correlation_id: CorrelationId,
        ) -> Result<SagaInstance<S>, SagaStoreError> {
            Ok(SagaInstance::new(saga_name, correlation_id))
        }

        async fn save<S: SagaState>(
            &self,
            _uow: &mut Self::Uow,
            _instance: &SagaInstance<S>,
        ) -> Result<(), SagaStoreError> {
            Ok(())
        }
    }
}
