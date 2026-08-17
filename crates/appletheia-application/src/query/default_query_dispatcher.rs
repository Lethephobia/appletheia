use crate::authorization::Authorizer;
use crate::projection::ProjectionConsistencyWaiter;
use crate::read_model::watch::{ReadModelWatchRegistrar, ReadModelWatchSelection};
use crate::request_context::RequestContext;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{QueryConsistency, QueryDispatcher, QueryDispatcherError, QueryHandler, QueryOptions};

pub struct DefaultQueryDispatcher<W, U, AZ, WR>
where
    W: ProjectionConsistencyWaiter,
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
{
    projection_consistency_waiter: W,
    uow_factory: U,
    authorizer: AZ,
    watch_registrar: WR,
}

impl<W, U, AZ, WR> DefaultQueryDispatcher<W, U, AZ, WR>
where
    W: ProjectionConsistencyWaiter,
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    AZ: Authorizer,
    WR: ReadModelWatchRegistrar,
{
    pub fn new(
        projection_consistency_waiter: W,
        uow_factory: U,
        authorizer: AZ,
        watch_registrar: WR,
    ) -> Self {
        Self {
            projection_consistency_waiter,
            uow_factory,
            authorizer,
            watch_registrar,
        }
    }
}

impl<W, U, AZ, WR> QueryDispatcher for DefaultQueryDispatcher<W, U, AZ, WR>
where
    W: ProjectionConsistencyWaiter,
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    AZ: Authorizer,
    WR: ReadModelWatchRegistrar,
{
    type Uow = U::Uow;

    async fn dispatch<H>(
        &self,
        handler: &H,
        request_context: &RequestContext,
        query: H::Query,
        options: QueryOptions,
    ) -> Result<H::Output, QueryDispatcherError<H::Error>>
    where
        H: QueryHandler<Uow = Self::Uow>,
    {
        let authorization_plan = handler
            .authorization_plan(&query)
            .map_err(QueryDispatcherError::Handler)?;
        self.authorizer
            .authorize(&request_context.principal, &authorization_plan)
            .await?;

        match options.consistency {
            QueryConsistency::Eventual => {}
            QueryConsistency::AfterMessage {
                message_id,
                timeout,
                poll_interval,
            } => {
                self.projection_consistency_waiter
                    .wait_for_message(
                        message_id,
                        timeout,
                        poll_interval,
                        H::PROJECTOR_DEPENDENCIES,
                    )
                    .await?;
            }
            QueryConsistency::AfterEvents {
                event_ids,
                timeout,
                poll_interval,
            } => {
                self.projection_consistency_waiter
                    .wait_for_events(
                        &event_ids,
                        timeout,
                        poll_interval,
                        H::PROJECTOR_DEPENDENCIES,
                    )
                    .await?;
            }
        }

        let mut uow = self.uow_factory.begin().await?;

        let result = handler.handle(&mut uow, request_context, query).await;
        match result {
            Ok(output) => {
                uow.commit().await?;
                if let Some(watch) = options.watch {
                    let selection = ReadModelWatchSelection::try_from_read_model(&output)?;
                    self.watch_registrar
                        .register(&watch.session_id, selection)
                        .await?;
                }
                Ok(output)
            }
            Err(operation_error) => {
                let operation_error = uow
                    .rollback_with_operation_error(operation_error)
                    .await
                    .map_err(QueryDispatcherError::UnitOfWork)?;
                Err(QueryDispatcherError::Handler(operation_error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    use appletheia_domain::EventId;
    use uuid::Uuid;

    use super::DefaultQueryDispatcher;
    use crate::authorization::{AuthorizationPlan, Authorizer, AuthorizerError};
    use crate::projection::{
        ProjectionConsistencyPollInterval, ProjectionConsistencyTimeout,
        ProjectionConsistencyWaitError, ProjectionConsistencyWaiter, ProjectorDependencies,
    };
    use crate::query::{
        Query, QueryConsistency, QueryDispatcher, QueryDispatcherError, QueryHandler, QueryName,
        QueryOptions, ReadModelWatchOptions,
    };
    use crate::read_model::watch::{
        ReadModelWatchRegistrar, ReadModelWatchRegistrationError, ReadModelWatchSelection,
        ReadModelWatchSessionId,
    };
    use crate::read_model::{
        ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
        ReadModelPartTree,
    };
    use crate::request_context::{MessageId, Principal, RequestContext};
    use crate::unit_of_work::{
        UnitOfWork, UnitOfWorkError, UnitOfWorkFactory, UnitOfWorkFactoryError,
    };

    #[derive(Debug, Default)]
    struct TestState {
        committed: AtomicBool,
        registrations: AtomicUsize,
        selection_calls: AtomicUsize,
    }

    struct TestUow {
        state: Arc<TestState>,
    }

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            self.state.committed.store(true, Ordering::SeqCst);
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    struct TestUowFactory {
        state: Arc<TestState>,
    }

    impl UnitOfWorkFactory for TestUowFactory {
        type Uow = TestUow;

        async fn begin(&self) -> Result<Self::Uow, UnitOfWorkFactoryError> {
            Ok(TestUow {
                state: Arc::clone(&self.state),
            })
        }
    }

    struct TestAuthorizer;

    impl Authorizer for TestAuthorizer {
        async fn authorize(
            &self,
            _principal: &Principal,
            _authorization_plan: &AuthorizationPlan,
        ) -> Result<(), AuthorizerError> {
            Ok(())
        }
    }

    struct TestProjectionConsistencyWaiter;

    impl ProjectionConsistencyWaiter for TestProjectionConsistencyWaiter {
        async fn wait_for_message(
            &self,
            _message_id: MessageId,
            _timeout: ProjectionConsistencyTimeout,
            _poll_interval: ProjectionConsistencyPollInterval,
            _projector_dependencies: ProjectorDependencies<'_>,
        ) -> Result<(), ProjectionConsistencyWaitError> {
            Ok(())
        }

        async fn wait_for_events(
            &self,
            _event_ids: &[EventId],
            _timeout: ProjectionConsistencyTimeout,
            _poll_interval: ProjectionConsistencyPollInterval,
            _projector_dependencies: ProjectorDependencies<'_>,
        ) -> Result<(), ProjectionConsistencyWaitError> {
            Ok(())
        }
    }

    struct TestWatchRegistrar {
        state: Arc<TestState>,
        reject: bool,
    }

    impl ReadModelWatchRegistrar for TestWatchRegistrar {
        async fn register(
            &self,
            session_id: &ReadModelWatchSessionId,
            _selection: ReadModelWatchSelection,
        ) -> Result<(), ReadModelWatchRegistrationError> {
            assert!(self.state.committed.load(Ordering::SeqCst));
            self.state.registrations.fetch_add(1, Ordering::SeqCst);
            if self.reject {
                return Err(ReadModelWatchRegistrationError::SessionNotFound(
                    session_id.clone(),
                ));
            }
            Ok(())
        }
    }

    struct TestQuery;

    impl Query for TestQuery {
        const NAME: QueryName = QueryName::new("test");
    }

    #[derive(Debug, thiserror::Error)]
    #[error("test query handler failed")]
    struct TestHandlerError;

    #[derive(Debug)]
    struct TestReadModel {
        value: usize,
        state: Arc<TestState>,
    }

    impl ReadModelObservationSource for TestReadModel {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModel for TestReadModel {
        const NAME: ReadModelName = ReadModelName::new("test");

        fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
            if let Some(read_model) = read_model {
                read_model
                    .state
                    .selection_calls
                    .fetch_add(1, Ordering::SeqCst);
            }
            Vec::new()
        }
    }

    struct TestHandler {
        state: Arc<TestState>,
    }

    impl QueryHandler for TestHandler {
        type Query = TestQuery;
        type Output = TestReadModel;
        type Error = TestHandlerError;
        type Uow = TestUow;

        async fn handle(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            _query: Self::Query,
        ) -> Result<Self::Output, Self::Error> {
            Ok(TestReadModel {
                value: 42,
                state: Arc::clone(&self.state),
            })
        }
    }

    fn request_context() -> RequestContext {
        RequestContext::new(
            crate::request_context::CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::System,
        )
        .expect("request context should be valid")
    }

    fn session_id() -> ReadModelWatchSessionId {
        ReadModelWatchSessionId::new()
    }

    fn dispatcher(
        state: Arc<TestState>,
        reject_registration: bool,
    ) -> DefaultQueryDispatcher<
        TestProjectionConsistencyWaiter,
        TestUowFactory,
        TestAuthorizer,
        TestWatchRegistrar,
    > {
        DefaultQueryDispatcher::new(
            TestProjectionConsistencyWaiter,
            TestUowFactory {
                state: Arc::clone(&state),
            },
            TestAuthorizer,
            TestWatchRegistrar {
                state,
                reject: reject_registration,
            },
        )
    }

    #[tokio::test]
    async fn registers_a_watch_selection_after_commit() {
        let state = Arc::new(TestState::default());
        let output = dispatcher(Arc::clone(&state), false)
            .dispatch(
                &TestHandler {
                    state: Arc::clone(&state),
                },
                &request_context(),
                TestQuery,
                QueryOptions {
                    consistency: QueryConsistency::Eventual,
                    watch: Some(ReadModelWatchOptions {
                        session_id: session_id(),
                    }),
                },
            )
            .await
            .expect("watchable query should succeed");

        assert_eq!(output.value, 42);
        assert!(state.committed.load(Ordering::SeqCst));
        assert_eq!(state.selection_calls.load(Ordering::SeqCst), 1);
        assert_eq!(state.registrations.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn returns_registration_error_after_the_read_unit_of_work_is_committed() {
        let state = Arc::new(TestState::default());
        let error = dispatcher(Arc::clone(&state), true)
            .dispatch(
                &TestHandler {
                    state: Arc::clone(&state),
                },
                &request_context(),
                TestQuery,
                QueryOptions {
                    consistency: QueryConsistency::Eventual,
                    watch: Some(ReadModelWatchOptions {
                        session_id: session_id(),
                    }),
                },
            )
            .await
            .expect_err("closed session should reject registration");

        assert!(state.committed.load(Ordering::SeqCst));
        assert!(matches!(
            error,
            QueryDispatcherError::WatchRegistration(
                ReadModelWatchRegistrationError::SessionNotFound(_)
            )
        ));
    }

    #[tokio::test]
    async fn skips_watch_selection_and_registration_without_watch_options() {
        let state = Arc::new(TestState::default());
        dispatcher(Arc::clone(&state), false)
            .dispatch(
                &TestHandler {
                    state: Arc::clone(&state),
                },
                &request_context(),
                TestQuery,
                QueryOptions::default(),
            )
            .await
            .expect("ordinary query should succeed");

        assert!(state.committed.load(Ordering::SeqCst));
        assert_eq!(state.selection_calls.load(Ordering::SeqCst), 0);
        assert_eq!(state.registrations.load(Ordering::SeqCst), 0);
    }
}
