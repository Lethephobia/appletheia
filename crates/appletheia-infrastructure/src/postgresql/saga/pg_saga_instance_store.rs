use crate::postgresql::saga::pg_saga_dispatched_command_row::PgSagaDispatchedCommandRow;
use crate::postgresql::saga::pg_saga_instance_row::PgSagaInstanceRow;
use crate::postgresql::unit_of_work::PgUnitOfWork;
use appletheia_application::request_context::{CorrelationId, MessageId};
use appletheia_application::saga::{
    SagaDispatchedCommand, SagaInstance, SagaInstanceStore, SagaInstanceStoreError, SagaNameOwned,
    SagaState, SagaStep,
};

#[derive(Debug)]
pub struct PgSagaInstanceStore;

impl PgSagaInstanceStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgSagaInstanceStore {
    fn default() -> Self {
        Self::new()
    }
}

impl PgSagaInstanceStore {
    async fn read_dispatched_commands<S: SagaStep>(
        uow: &mut PgUnitOfWork,
        saga_instance_id: uuid::Uuid,
    ) -> Result<Vec<SagaDispatchedCommand<S>>, SagaInstanceStoreError> {
        let transaction = uow.transaction_mut();

        let rows = sqlx::query_as::<_, PgSagaDispatchedCommandRow>(
            r#"
            SELECT message_id, command_name, step
            FROM saga_dispatched_commands
            WHERE saga_instance_id = $1
            ORDER BY message_id ASC
            "#,
        )
        .bind(saga_instance_id)
        .fetch_all(transaction.as_mut())
        .await
        .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;

        rows.into_iter()
            .map(PgSagaDispatchedCommandRow::try_into_dispatched_command::<S>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))
    }
}

impl SagaInstanceStore for PgSagaInstanceStore {
    type Uow = PgUnitOfWork;

    async fn find_by_correlation_id<S: SagaState, T: SagaStep>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        correlation_id: CorrelationId,
    ) -> Result<Option<SagaInstance<S, T>>, SagaInstanceStoreError> {
        let transaction = uow.transaction_mut();

        let saga_name_value = saga_name.value();
        let correlation_id_value = correlation_id.value();

        let row = sqlx::query_as::<_, PgSagaInstanceRow>(
            r#"
            SELECT
              id,
              correlation_id,
              start_event_id,
              state
            FROM saga_instances
            WHERE saga_name = $1
              AND correlation_id = $2
            FOR UPDATE
            "#,
        )
        .bind(saga_name_value)
        .bind(correlation_id_value)
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let dispatched_commands = Self::read_dispatched_commands::<T>(uow, row.id).await?;

        row.try_into_instance::<S, T>(saga_name, correlation_id, dispatched_commands)
            .map(Some)
            .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))
    }

    async fn find_by_dispatched_command_message_id<S: SagaState, T: SagaStep>(
        &self,
        uow: &mut Self::Uow,
        saga_name: SagaNameOwned,
        dispatched_command_message_id: MessageId,
    ) -> Result<Option<SagaInstance<S, T>>, SagaInstanceStoreError> {
        let transaction = uow.transaction_mut();

        let row = sqlx::query_as::<_, PgSagaInstanceRow>(
            r#"
            SELECT
              si.id,
              si.correlation_id,
              si.start_event_id,
              si.state
            FROM saga_instances si
            JOIN saga_dispatched_commands sdc
              ON sdc.saga_instance_id = si.id
            WHERE si.saga_name = $1
              AND sdc.message_id = $2
            FOR UPDATE OF si
            "#,
        )
        .bind(saga_name.value())
        .bind(dispatched_command_message_id.value())
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let dispatched_commands = Self::read_dispatched_commands::<T>(uow, row.id).await?;
        let correlation_id = CorrelationId::from(row.correlation_id);

        row.try_into_instance::<S, T>(saga_name, correlation_id, dispatched_commands)
            .map(Some)
            .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))
    }

    async fn save<S: SagaState, T: SagaStep>(
        &self,
        uow: &mut Self::Uow,
        instance: &SagaInstance<S, T>,
    ) -> Result<(), SagaInstanceStoreError> {
        let transaction = uow.transaction_mut();

        let saga_instance_id_value = instance.saga_instance_id.value();

        let state_json = match instance.state.as_ref() {
            Some(state) => {
                Some(serde_json::to_value(state).map_err(SagaInstanceStoreError::StateSerialize)?)
            }
            None => None,
        };

        let persisted_saga_instance_id = sqlx::query_scalar::<_, uuid::Uuid>(
            r#"
            INSERT INTO saga_instances (
              id,
              saga_name,
              correlation_id,
              start_event_id,
              state
            ) VALUES (
              $1,
              $2,
              $3,
              $4,
              $5
            )
            ON CONFLICT (id) DO UPDATE SET
              state = EXCLUDED.state
            RETURNING id
            "#,
        )
        .bind(saga_instance_id_value)
        .bind(instance.saga_name.value())
        .bind(instance.correlation_id.value())
        .bind(instance.start_event_id.value())
        .bind(state_json)
        .fetch_one(transaction.as_mut())
        .await
        .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;

        for command in &instance.uncommitted_commands {
            let origin = command
                .saga_origin
                .as_ref()
                .ok_or(SagaInstanceStoreError::MissingCommandOrigin)?;
            if origin.saga_name != instance.saga_name
                || origin.saga_instance_id != instance.saga_instance_id
            {
                return Err(SagaInstanceStoreError::CommandOriginMismatch);
            }
            sqlx::query(
                r#"
                INSERT INTO saga_dispatched_commands (
                  saga_instance_id,
                  message_id,
                  command_name,
                  step
                ) VALUES (
                  $1,
                  $2,
                  $3,
                  $4
                )
                ON CONFLICT DO NOTHING
                "#,
            )
            .bind(persisted_saga_instance_id)
            .bind(command.message_id.value())
            .bind(command.command_name.value())
            .bind(origin.step.value().clone())
            .execute(transaction.as_mut())
            .await
            .map_err(|source| SagaInstanceStoreError::Persistence(Box::new(source)))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod counter {
        use appletheia_domain::{
            Aggregate, AggregateApply, AggregateCore, AggregateError, AggregateId, AggregateState,
            AggregateStateError, AggregateType, EventName, EventPayload, ReferenceIndexes,
            UniqueConstraints,
        };
        use serde::{Deserialize, Serialize};
        use std::fmt::{self, Display};
        use thiserror::Error;
        use uuid::Uuid;

        #[derive(Debug, Error)]
        pub(super) enum CounterIdError {
            #[error("nil uuid is not allowed")]
            NilUuid,
        }

        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub(super) struct CounterId(Uuid);

        impl AggregateId for CounterId {
            type Error = CounterIdError;

            fn new() -> Self {
                Self(Uuid::now_v7())
            }

            fn value(&self) -> Uuid {
                self.0
            }

            fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
                if value.is_nil() {
                    return Err(CounterIdError::NilUuid);
                }

                Ok(Self(value))
            }
        }

        impl Display for CounterId {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        #[derive(Debug, Error)]
        pub(super) enum CounterStateError {
            #[error(transparent)]
            AggregateState(#[from] AggregateStateError),
        }

        #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
        pub(super) struct CounterState {
            id: CounterId,
        }

        impl UniqueConstraints<CounterStateError> for CounterState {}

        impl ReferenceIndexes<CounterStateError> for CounterState {}

        impl AggregateState for CounterState {
            type Error = CounterStateError;
        }

        #[derive(Debug, Error)]
        pub(super) enum CounterEventPayloadError {
            #[error(transparent)]
            Serde(#[from] serde_json::Error),
        }

        #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
        #[serde(tag = "type", content = "data", rename_all = "snake_case")]
        pub(super) enum CounterEventPayload {
            Opened,
        }

        impl EventPayload for CounterEventPayload {
            type Error = CounterEventPayloadError;

            fn name(&self) -> EventName {
                match self {
                    Self::Opened => EventName::new("opened"),
                }
            }
        }

        #[derive(Debug, Error)]
        pub(super) enum CounterError {
            #[error(transparent)]
            Aggregate(#[from] AggregateError<CounterId>),

            #[error(transparent)]
            State(#[from] CounterStateError),
        }

        #[derive(Clone, Debug, Default)]
        pub(super) struct Counter {
            core: AggregateCore<CounterId, CounterState, CounterEventPayload>,
        }

        impl AggregateApply<CounterEventPayload, CounterError> for Counter {
            fn apply(&mut self, payload: &CounterEventPayload) -> Result<(), CounterError> {
                match payload {
                    CounterEventPayload::Opened => {
                        self.set_state(Some(CounterState {
                            id: CounterId::try_from_uuid(Uuid::now_v7())
                                .expect("generated uuid should be valid"),
                        }));
                    }
                }

                Ok(())
            }
        }

        impl Aggregate for Counter {
            type Id = CounterId;
            type State = CounterState;
            type EventPayload = CounterEventPayload;
            type Error = CounterError;

            const TYPE: AggregateType = AggregateType::new("counter");

            fn new() -> Self {
                Self {
                    core: AggregateCore::new(),
                }
            }

            fn from_id(id: Self::Id) -> Self {
                Self {
                    core: AggregateCore::from_id(id),
                }
            }

            fn core(&self) -> &AggregateCore<Self::Id, Self::State, Self::EventPayload> {
                &self.core
            }

            fn core_mut(
                &mut self,
            ) -> &mut AggregateCore<Self::Id, Self::State, Self::EventPayload> {
                &mut self.core
            }
        }
    }

    use crate::postgresql::PgUnitOfWorkFactory;
    use crate::postgresql::outbox::command::PgCommandOutboxEnqueuer;
    use crate::postgresql::saga::{
        PgSagaInstanceStore, PgSagaProcessedCommandFailureStore, PgSagaProcessedEventStore,
    };
    use appletheia_application::command::{
        Command, CommandAttemptCount, CommandEnvelope, CommandFailedAt, CommandFailureEnvelope,
        CommandFailureId, CommandName, CommandOptions, CommandTerminalReason,
    };
    use appletheia_application::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
        SerializedEventPayload,
    };
    use appletheia_application::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia_application::saga::*;
    use appletheia_application::unit_of_work::{UnitOfWork, UnitOfWorkFactory};
    use appletheia_domain::{
        Aggregate, AggregateVersion, EventId, EventName, EventOccurredAt, EventPayload,
    };
    use counter::{Counter, CounterEventPayload};
    use serde::{Deserialize, Serialize};
    use sqlx::PgPool;
    use uuid::Uuid;

    type Route<'a> = SagaRoute<'a, State, Step, Error>;

    #[derive(Debug, Default, Serialize, Deserialize)]
    struct State {
        calls: u32,
        closed: bool,
    }

    impl SagaState for State {}
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    enum Step {
        First,
        Second,
    }

    impl SagaStep for Step {}
    #[derive(Debug, thiserror::Error)]
    enum Error {
        #[error(transparent)]
        Context(#[from] SagaContextError),
        #[error("injected failure")]
        Injected,
    }

    #[derive(Serialize, Deserialize)]
    struct FollowUp {}

    impl Command for FollowUp {
        const NAME: CommandName = CommandName::new("follow_up");
    }

    fn runner(pool: &PgPool) -> impl SagaRunner {
        DefaultSagaRunner::new(
            PgSagaInstanceStore::new(),
            PgSagaProcessedEventStore::new(),
            PgSagaProcessedCommandFailureStore::new(),
            PgCommandOutboxEnqueuer::new(),
            PgUnitOfWorkFactory::new(pool.clone()),
        )
    }

    fn event() -> EventEnvelope {
        let message_id = MessageId::new();
        let correlation_id = CorrelationId::from(message_id.value());
        let payload = CounterEventPayload::Opened;
        EventEnvelope {
            event_sequence: EventSequence::try_from(1).unwrap(),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(Counter::TYPE),
            aggregate_id: AggregateIdValue::from(Uuid::now_v7()),
            aggregate_version: AggregateVersion::try_from(1).unwrap(),
            event_name: EventNameOwned::from(payload.name()),
            payload: SerializedEventPayload::try_from(payload.into_json_value().unwrap()).unwrap(),
            occurred_at: EventOccurredAt::now(),
            correlation_id,
            causation_id: CausationId::from(message_id),
            context: RequestContext::new(correlation_id, message_id, Principal::System).unwrap(),
        }
    }

    async fn count(pool: &PgPool, table: &str) -> i64 {
        let sql = match table {
            "saga_instances" => "SELECT count(*) FROM saga_instances",
            "saga_dispatched_commands" => "SELECT count(*) FROM saga_dispatched_commands",
            "command_outbox" => "SELECT count(*) FROM command_outbox",
            "saga_processed_events" => "SELECT count(*) FROM saga_processed_events",
            "saga_processed_command_failures" => {
                "SELECT count(*) FROM saga_processed_command_failures"
            }
            _ => panic!("unknown test table"),
        };
        sqlx::query_scalar(sql).fetch_one(pool).await.unwrap()
    }

    async fn dispatched(
        pool: &PgPool,
        correlation: CorrelationId,
    ) -> (SagaInstance<State, Step>, CommandEnvelope) {
        let factory = PgUnitOfWorkFactory::new(pool.clone());
        let mut uow = factory.begin().await.unwrap();
        let instance = PgSagaInstanceStore::new()
            .find_by_correlation_id::<State, Step>(
                &mut uow,
                SagaNameOwned::from(SagaName::new("counter_saga")),
                correlation,
            )
            .await
            .unwrap()
            .unwrap();
        let saved = &instance.dispatched_commands[0];
        let mut command = CommandEnvelope::new(
            &FollowUp {},
            correlation,
            CausationId::from(instance.start_event_id),
            CommandOptions::default(),
        )
        .unwrap()
        .with_saga_origin(SagaCommandOrigin {
            saga_name: instance.saga_name.clone(),
            saga_instance_id: instance.saga_instance_id,
            step: SerializedSagaStep::new(saved.step).unwrap(),
        });
        command.message_id = saved.message_id;
        uow.commit().await.unwrap();
        (instance, command)
    }

    fn failure(command: &CommandEnvelope) -> CommandFailureEnvelope {
        CommandFailureEnvelope::new(
            command,
            command.saga_origin.clone().unwrap(),
            CommandTerminalReason::NonRetryable,
            CommandAttemptCount::first(),
            CommandFailedAt::now(),
        )
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn competing_new_instance_cannot_overwrite_existing_correlation(pool: PgPool) {
        let factory = PgUnitOfWorkFactory::new(pool.clone());
        let store = PgSagaInstanceStore::new();
        let input = event();
        let name = SagaNameOwned::from(SagaName::new("counter_saga"));
        let mut first =
            SagaInstance::<State, Step>::new(name.clone(), input.correlation_id, input.event_id);
        first.state = Some(State {
            calls: 1,
            closed: false,
        });
        let mut competing =
            SagaInstance::<State, Step>::new(name.clone(), input.correlation_id, EventId::new());
        competing.state = Some(State {
            calls: 99,
            closed: false,
        });
        let mut initial_uow = factory.begin().await.unwrap();
        store.save(&mut initial_uow, &first).await.unwrap();
        initial_uow.commit().await.unwrap();
        let mut competing_uow = factory.begin().await.unwrap();
        assert!(store.save(&mut competing_uow, &competing).await.is_err());
        competing_uow.rollback().await.unwrap();
        let mut read_uow = factory.begin().await.unwrap();
        let saved = store
            .find_by_correlation_id::<State, Step>(&mut read_uow, name, input.correlation_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(saved.saga_instance_id, first.saga_instance_id);
        assert_eq!(saved.state.unwrap().calls, 1);
        read_uow.commit().await.unwrap();
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn existing_correlation_skips_start_without_state_guard(pool: PgPool) {
        let definition = SagaDefinition::<State, Step, Error>::new(
            SagaName::new("counter_saga"),
            [Route::starts_on::<Counter, _>(
                EventName::new("opened"),
                Step::First,
                |ctx, _| {
                    ctx.set_state(State {
                        calls: 1,
                        closed: true,
                    });
                    ctx.append_command(&FollowUp {})?;
                    ctx.append_command(&FollowUp {})?;
                    Ok(())
                },
            )],
        )
        .unwrap();
        let run = runner(&pool);
        let input = event();
        assert!(matches!(
            run.handle_event(&definition, &input).await.unwrap(),
            SagaEventRunReport::Processed { .. }
        ));
        assert_eq!(
            run.handle_event(&definition, &input).await.unwrap(),
            SagaEventRunReport::AlreadyStarted
        );
        let mut later = input.clone();
        later.event_id = EventId::new();
        assert_eq!(
            run.handle_event(&definition, &later).await.unwrap(),
            SagaEventRunReport::AlreadyStarted
        );
        assert_eq!(count(&pool, "saga_instances").await, 1);
        assert_eq!(count(&pool, "saga_dispatched_commands").await, 2);
        assert_eq!(count(&pool, "command_outbox").await, 2);
        assert_eq!(count(&pool, "saga_processed_events").await, 1);
        let (saved, _) = dispatched(&pool, input.correlation_id).await;
        assert_eq!(saved.state.unwrap().calls, 1);
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn unmatched_failure_is_recorded_without_changing_state_or_dispatching(pool: PgPool) {
        let definition =
            SagaDefinitionBuilder::<State, Step, Error>::new(SagaName::new("counter_saga"))
                .add_start_step(Step::First)
                .on::<Counter>(EventName::new("opened"))
                .handle(|ctx, _| {
                    ctx.set_state(State::default());
                    ctx.append_command(&FollowUp {})?;
                    Ok(())
                })
                .build()
                .unwrap();
        let run = runner(&pool);
        let input = event();
        run.handle_event(&definition, &input).await.unwrap();
        let (_, command) = dispatched(&pool, input.correlation_id).await;
        let notification = failure(&command);
        assert_eq!(
            run.handle_command_failure(&definition, &notification)
                .await
                .unwrap(),
            SagaCommandFailureRunReport::NoMatchingRoute
        );
        assert_eq!(count(&pool, "saga_processed_command_failures").await, 1);
        assert_eq!(count(&pool, "command_outbox").await, 1);
        let (saved, _) = dispatched(&pool, input.correlation_id).await;
        assert_eq!(saved.state.unwrap().calls, 0);
        assert_eq!(saved.dispatched_commands.len(), 1);
        assert_eq!(
            run.handle_command_failure(&definition, &notification)
                .await
                .unwrap(),
            SagaCommandFailureRunReport::AlreadyProcessed
        );
        let mut republished = notification.clone();
        republished.failure_id = CommandFailureId::new();
        assert_eq!(
            run.handle_command_failure(&definition, &republished)
                .await
                .unwrap(),
            SagaCommandFailureRunReport::AlreadyProcessed
        );
        assert_eq!(count(&pool, "saga_processed_command_failures").await, 1);
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn callback_error_rolls_back_state_processed_input_and_commands(pool: PgPool) {
        let definition = SagaDefinition::<State, Step, Error>::new(
            SagaName::new("counter_saga"),
            [Route::starts_on::<Counter, _>(
                EventName::new("opened"),
                Step::First,
                |ctx, _| {
                    ctx.set_state(State::default());
                    ctx.append_command(&FollowUp {})?;
                    Err(Error::Injected)
                },
            )],
        )
        .unwrap();
        let run = runner(&pool);
        let input = event();
        for _ in 0..2 {
            assert!(matches!(
                run.handle_event(&definition, &input).await,
                Err(SagaRunnerError::Route(SagaRouteError::Handler(
                    Error::Injected
                )))
            ));
        }
        for table in [
            "saga_instances",
            "saga_processed_events",
            "saga_dispatched_commands",
            "command_outbox",
        ] {
            assert_eq!(count(&pool, table).await, 0);
        }
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn failure_origin_validation_and_duplicate_delivery_do_not_repeat_compensation(
        pool: PgPool,
    ) {
        let definition = SagaDefinition::<State, Step, Error>::new(
            SagaName::new("counter_saga"),
            [
                Route::starts_on::<Counter, _>(EventName::new("opened"), Step::First, |ctx, _| {
                    ctx.set_state(State::default());
                    ctx.append_command(&FollowUp {})?;
                    Ok(())
                }),
                Route::on_command_failed(Step::First, Step::Second, |ctx, _| {
                    ctx.state_required_mut()?.calls += 1;
                    ctx.append_command(&FollowUp {})?;
                    Ok(())
                }),
            ],
        )
        .unwrap();
        let run = runner(&pool);
        let input = event();
        run.handle_event(&definition, &input).await.unwrap();
        let (_, command) = dispatched(&pool, input.correlation_id).await;
        let notification = failure(&command);
        let mut wrong_id = notification.clone();
        wrong_id.origin.saga_instance_id = SagaInstanceId::new();
        let mut wrong_step = notification.clone();
        wrong_step.origin.step = SerializedSagaStep::new(Step::Second).unwrap();
        let mut wrong_name = notification.clone();
        wrong_name.command_name = CommandName::new("other").into();
        for invalid in [wrong_id, wrong_step, wrong_name] {
            assert_eq!(
                run.handle_command_failure(&definition, &invalid)
                    .await
                    .unwrap(),
                SagaCommandFailureRunReport::CommandNotOwned
            );
        }
        assert_eq!(count(&pool, "saga_processed_command_failures").await, 0);
        assert!(matches!(
            run.handle_command_failure(&definition, &notification)
                .await
                .unwrap(),
            SagaCommandFailureRunReport::Processed { .. }
        ));
        assert_eq!(
            run.handle_command_failure(&definition, &notification)
                .await
                .unwrap(),
            SagaCommandFailureRunReport::AlreadyProcessed
        );
        let mut republished = notification.clone();
        republished.failure_id = CommandFailureId::new();
        assert_eq!(
            run.handle_command_failure(&definition, &republished)
                .await
                .unwrap(),
            SagaCommandFailureRunReport::AlreadyProcessed
        );
        assert_eq!(count(&pool, "command_outbox").await, 2);
        let cause: Uuid = sqlx::query_scalar(
            "SELECT causation_id FROM command_outbox WHERE saga_step = '\"Second\"'::jsonb",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(cause, notification.failure_id.value());
        let (saved, _) = dispatched(&pool, input.correlation_id).await;
        assert_eq!(saved.state.unwrap().calls, 1);
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn continuation_requires_owned_command_and_late_inputs_remain_processable(pool: PgPool) {
        let definition =
            SagaDefinitionBuilder::<State, Step, Error>::new(SagaName::new("counter_saga"))
                .add_start_step(Step::First)
                .on::<Counter>(EventName::new("opened"))
                .handle(|ctx, _| {
                    ctx.set_state(State::default());
                    ctx.append_command(&FollowUp {})?;
                    Ok(())
                })
                .add_step(Step::Second)
                .on::<Counter>(Step::First, EventName::new("opened"))
                .handle(|ctx, _| {
                    ctx.state_required_mut()?.calls += 1;
                    ctx.append_command(&FollowUp {})?;
                    Ok(())
                })
                .build()
                .unwrap();
        let run = runner(&pool);
        let input = event();
        run.handle_event(&definition, &input).await.unwrap();
        let (_, command) = dispatched(&pool, input.correlation_id).await;
        let mut follow_up = input.clone();
        follow_up.event_id = EventId::new();
        assert_eq!(
            run.handle_event(&definition, &follow_up).await.unwrap(),
            SagaEventRunReport::AlreadyStarted
        );
        follow_up.causation_id = CausationId::from(command.message_id);
        assert!(matches!(
            run.handle_event(&definition, &follow_up).await.unwrap(),
            SagaEventRunReport::Processed { .. }
        ));
        assert_eq!(
            run.handle_event(&definition, &follow_up).await.unwrap(),
            SagaEventRunReport::AlreadyProcessed
        );
        follow_up.event_id = EventId::new();
        assert!(matches!(
            run.handle_event(&definition, &follow_up).await.unwrap(),
            SagaEventRunReport::Processed { .. }
        ));
        let (saved, _) = dispatched(&pool, input.correlation_id).await;
        assert_eq!(saved.state.as_ref().unwrap().calls, 2);
        let second_command = saved
            .dispatched_commands
            .iter()
            .find(|command| command.step == Step::Second)
            .unwrap();
        let mut unmatched = input.clone();
        unmatched.event_id = EventId::new();
        unmatched.causation_id = CausationId::from(second_command.message_id);
        assert_eq!(
            run.handle_event(&definition, &unmatched).await.unwrap(),
            SagaEventRunReport::NoMatchingRoute
        );
        assert_eq!(count(&pool, "command_outbox").await, 3);
    }

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn failure_handler_and_outbox_errors_roll_back_and_can_be_retried(pool: PgPool) {
        let start = SagaDefinition::<State, Step, Error>::new(
            SagaName::new("counter_saga"),
            [Route::starts_on::<Counter, _>(
                EventName::new("opened"),
                Step::First,
                |ctx, _| {
                    ctx.set_state(State::default());
                    ctx.append_command(&FollowUp {})?;
                    Ok(())
                },
            )],
        )
        .unwrap();
        let run = runner(&pool);
        let input = event();
        run.handle_event(&start, &input).await.unwrap();
        let (_, command) = dispatched(&pool, input.correlation_id).await;
        let notification = failure(&command);
        let refused = SagaDefinition::<State, Step, Error>::new(
            SagaName::new("counter_saga"),
            [Route::on_command_failed(
                Step::First,
                Step::Second,
                |ctx, _| {
                    ctx.state_required_mut()?.calls += 1;
                    ctx.append_command(&FollowUp {})?;
                    Err(Error::Injected)
                },
            )],
        )
        .unwrap();
        assert!(
            run.handle_command_failure(&refused, &notification)
                .await
                .is_err()
        );
        assert_eq!(count(&pool, "saga_processed_command_failures").await, 0);
        let (unchanged, _) = dispatched(&pool, input.correlation_id).await;
        assert_eq!(unchanged.state.unwrap().calls, 0);
        let recovery = SagaDefinition::<State, Step, Error>::new(
            SagaName::new("counter_saga"),
            [Route::on_command_failed(
                Step::First,
                Step::Second,
                |ctx, _| {
                    ctx.state_required_mut()?.calls += 1;
                    ctx.append_command(&FollowUp {})?;
                    Ok(())
                },
            )],
        )
        .unwrap();
        sqlx::query("ALTER TABLE command_outbox ADD CONSTRAINT reject_compensation CHECK (saga_step <> '\"Second\"'::jsonb)").execute(&pool).await.unwrap();
        assert!(matches!(
            run.handle_command_failure(&recovery, &notification).await,
            Err(SagaRunnerError::CommandOutbox(_))
        ));
        assert_eq!(count(&pool, "saga_processed_command_failures").await, 0);
        assert_eq!(count(&pool, "saga_dispatched_commands").await, 1);
        let (unchanged_again, _) = dispatched(&pool, input.correlation_id).await;
        assert_eq!(unchanged_again.state.unwrap().calls, 0);
        sqlx::query("ALTER TABLE command_outbox DROP CONSTRAINT reject_compensation")
            .execute(&pool)
            .await
            .unwrap();
        assert!(matches!(
            run.handle_command_failure(&recovery, &notification)
                .await
                .unwrap(),
            SagaCommandFailureRunReport::Processed { .. }
        ));
    }
}
