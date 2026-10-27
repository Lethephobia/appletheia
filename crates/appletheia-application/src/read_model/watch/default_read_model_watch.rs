use crate::Retryability;
use crate::query::{QueryDispatcher, QueryHandler};
use crate::read_model::ReadModelDocument;
use crate::request_context::RequestContext;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::default_read_model_watch_status::DefaultReadModelWatchStatus;
use super::{
    DefaultReadModelWatchDependencies, ReadModelWatch, ReadModelWatchEndpoint, ReadModelWatchError,
    ReadModelWatchGenerationRegistrar, ReadModelWatchGenerationRegistrarFactory, ReadModelWatchId,
    ReadModelWatchReceiver, ReadModelWatchRegistry, ReadModelWatchResult, ReadModelWatchSignal,
};

pub struct DefaultReadModelWatch<'a, D, F, R, E, G, H>
where
    D: QueryDispatcher,
    F: UnitOfWorkFactory<Uow = D::Uow>,
    R: ReadModelWatchRegistry<Uow = D::Uow>,
    E: ReadModelWatchEndpoint,
    G: ReadModelWatchGenerationRegistrarFactory<Uow = D::Uow>,
    H: QueryHandler<Uow = D::Uow>,
{
    query_dispatcher: &'a D,
    uow_factory: &'a F,
    registry: &'a R,
    endpoint: &'a E,
    generation_registrar_factory: &'a G,
    handler: &'a H,
    request_context: &'a RequestContext,
    query: H::Query,
    watch_id: ReadModelWatchId,
    receiver: E::Receiver,
    status: DefaultReadModelWatchStatus,
}

impl<'a, D, F, R, E, G, H> DefaultReadModelWatch<'a, D, F, R, E, G, H>
where
    D: QueryDispatcher,
    F: UnitOfWorkFactory<Uow = D::Uow>,
    R: ReadModelWatchRegistry<Uow = D::Uow>,
    E: ReadModelWatchEndpoint,
    G: ReadModelWatchGenerationRegistrarFactory<Uow = D::Uow>,
    H: QueryHandler<Uow = D::Uow>,
{
    /// Creates a watch whose notification route and database registration already exist.
    /// The first call to `next` queries and establishes target coverage before returning data.
    pub fn new(
        dependencies: DefaultReadModelWatchDependencies<'a, D, F, R, E, G>,
        handler: &'a H,
        request_context: &'a RequestContext,
        query: H::Query,
        watch_id: ReadModelWatchId,
        receiver: E::Receiver,
    ) -> Self {
        Self {
            query_dispatcher: dependencies.query_dispatcher,
            uow_factory: dependencies.uow_factory,
            registry: dependencies.registry,
            endpoint: dependencies.endpoint,
            generation_registrar_factory: dependencies.generation_registrar_factory,
            handler,
            request_context,
            query,
            watch_id,
            receiver,
            status: DefaultReadModelWatchStatus::RefreshRequired,
        }
    }

    async fn refresh(&mut self) -> Result<H::Output, ReadModelWatchError<H::Error>> {
        let endpoint_id = self.endpoint.endpoint_id();
        let mut uow = self.uow_factory.begin().await?;
        let mut registrar = match self
            .generation_registrar_factory
            .begin_generation(&mut uow, endpoint_id, self.watch_id)
            .await
        {
            Ok(registrar) => registrar,
            Err(source) => {
                return Err(uow
                    .rollback_with_operation_error(ReadModelWatchError::GenerationRegistrarFactory(
                        source,
                    ))
                    .await?);
            }
        };
        uow.commit().await?;

        let initial_document = self
            .query_dispatcher
            .dispatch(self.handler, self.request_context, self.query.clone())
            .await?;
        let mut targets = initial_document.read_model_refs();
        loop {
            let mut uow = self.uow_factory.begin().await?;
            if let Err(source) = registrar.register_targets(&mut uow, &targets).await {
                return Err(uow
                    .rollback_with_operation_error(ReadModelWatchError::GenerationRegistrar(source))
                    .await?);
            }
            uow.commit().await?;

            let document = self
                .query_dispatcher
                .dispatch(self.handler, self.request_context, self.query.clone())
                .await?;
            targets = document.read_model_refs();
            let mut uow = self.uow_factory.begin().await?;
            let covered = match registrar.contains_targets(&mut uow, &targets).await {
                Ok(covered) => covered,
                Err(source) => {
                    return Err(uow
                        .rollback_with_operation_error(ReadModelWatchError::GenerationRegistrar(
                            source,
                        ))
                        .await?);
                }
            };
            if !covered {
                uow.commit().await?;
                continue;
            }
            if let Err(source) = registrar.complete(&mut uow).await {
                return Err(uow
                    .rollback_with_operation_error(ReadModelWatchError::GenerationRegistrar(source))
                    .await?);
            }
            uow.commit().await?;
            return Ok(document);
        }
    }
}

impl<D, F, R, E, G, H> ReadModelWatch for DefaultReadModelWatch<'_, D, F, R, E, G, H>
where
    D: QueryDispatcher,
    F: UnitOfWorkFactory<Uow = D::Uow>,
    R: ReadModelWatchRegistry<Uow = D::Uow>,
    E: ReadModelWatchEndpoint,
    G: ReadModelWatchGenerationRegistrarFactory<Uow = D::Uow>,
    H: QueryHandler<Uow = D::Uow>,
{
    type Document = H::Output;
    type HandlerError = H::Error;

    fn watch_id(&self) -> ReadModelWatchId {
        self.watch_id
    }

    async fn next(
        &mut self,
    ) -> Result<ReadModelWatchResult<Self::Document>, ReadModelWatchError<Self::HandlerError>> {
        match self.status {
            DefaultReadModelWatchStatus::Closed => return Ok(ReadModelWatchResult::Closed),
            DefaultReadModelWatchStatus::Closing => {
                self.close().await?;
                return Ok(ReadModelWatchResult::Closed);
            }
            DefaultReadModelWatchStatus::Waiting => match self.receiver.receive().await? {
                ReadModelWatchSignal::Changed => {
                    self.status = DefaultReadModelWatchStatus::RefreshRequired;
                }
                ReadModelWatchSignal::Closed => {
                    self.close().await?;
                    return Ok(ReadModelWatchResult::Closed);
                }
            },
            DefaultReadModelWatchStatus::RefreshRequired => {}
        }
        let document = match self.refresh().await {
            Ok(document) => document,
            Err(ReadModelWatchError::QueryDispatcher(error)) if !error.is_retryable() => {
                if let Err(cleanup_error) = self.close().await {
                    return Err(ReadModelWatchError::OperationAndCleanupFailed {
                        operation_error: error,
                        cleanup_error: Box::new(cleanup_error),
                    });
                }
                return Err(ReadModelWatchError::QueryDispatcher(error));
            }
            Err(error) => return Err(error),
        };
        self.status = DefaultReadModelWatchStatus::Waiting;
        Ok(ReadModelWatchResult::Document(document))
    }

    async fn close(&mut self) -> Result<(), ReadModelWatchError<Self::HandlerError>> {
        if self.status == DefaultReadModelWatchStatus::Closed {
            return Ok(());
        }
        self.status = DefaultReadModelWatchStatus::Closing;
        let mut uow = self.uow_factory.begin().await?;
        if let Err(source) = self
            .registry
            .remove_watch(&mut uow, self.endpoint.endpoint_id(), self.watch_id)
            .await
        {
            return Err(uow
                .rollback_with_operation_error(ReadModelWatchError::Registry(source))
                .await?);
        }
        uow.commit().await?;
        self.endpoint.remove_watch(self.watch_id).await?;
        self.status = DefaultReadModelWatchStatus::Closed;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::future::pending;
    use std::io;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use tokio::sync::watch;
    use tokio::time::timeout;
    use uuid::Uuid;

    use super::*;
    use crate::query::{Query, QueryDispatcherError, QueryName};
    use crate::read_model::watch::{
        DefaultReadModelWatchGenerationRegistrarFactory, DefaultReadModelWatchReceiver,
        ReadModelWatchEndpointError, ReadModelWatchEndpointId, ReadModelWatchGeneration,
        ReadModelWatchRegistryError,
    };
    use crate::read_model::{
        ReadModelDocumentError, ReadModelId, ReadModelRef, ReadModelResourceDocument,
    };
    use crate::request_context::{CorrelationId, MessageId, Principal};
    use crate::unit_of_work::{UnitOfWorkError, UnitOfWorkFactoryError};

    #[derive(Default)]
    struct State {
        generation: ReadModelWatchGeneration,
        targets: HashMap<ReadModelWatchGeneration, HashSet<ReadModelRef>>,
        operations: Vec<&'static str>,
        fail_remove: bool,
        fail_begin: bool,
        fail_register: bool,
        fail_commit: bool,
        fail_cleanup: bool,
        fail_receiver: bool,
    }

    #[derive(Clone, Default)]
    struct Backend(Arc<Mutex<State>>);

    enum Operation {
        Register,
        Generation(ReadModelWatchGeneration),
        Targets(ReadModelWatchGeneration, Vec<ReadModelRef>),
        Complete(ReadModelWatchGeneration),
        Remove,
    }

    struct TestUow {
        backend: Backend,
        operations: Vec<Operation>,
    }

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            let mut state = self.backend.0.lock().unwrap();
            if state.fail_commit {
                return Err(UnitOfWorkError::CommitFailed(Box::new(io::Error::other(
                    "commit failed",
                ))));
            }
            for operation in self.operations {
                match operation {
                    Operation::Register => state.operations.push("register_db"),
                    Operation::Generation(generation) => {
                        state.generation = generation;
                        state.operations.push("begin");
                    }
                    Operation::Targets(generation, targets) => {
                        state.targets.entry(generation).or_default().extend(targets);
                        state.operations.push("register");
                    }
                    Operation::Complete(generation) => {
                        state.targets.retain(|key, _| *key >= generation);
                        state.operations.push("complete");
                    }
                    Operation::Remove => {
                        state.targets.clear();
                        state.operations.push("remove_db");
                    }
                }
            }
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            self.backend.0.lock().unwrap().operations.push("rollback");
            Ok(())
        }
    }

    impl UnitOfWorkFactory for Backend {
        type Uow = TestUow;

        async fn begin(&self) -> Result<TestUow, UnitOfWorkFactoryError> {
            if self.0.lock().unwrap().fail_begin {
                return Err(UnitOfWorkFactoryError::BeginFailed(Box::new(
                    io::Error::other("begin failed"),
                )));
            }
            Ok(TestUow {
                backend: self.clone(),
                operations: Vec::new(),
            })
        }
    }

    impl ReadModelWatchRegistry for Backend {
        type Uow = TestUow;

        async fn register_watch(
            &self,
            uow: &mut TestUow,
            _: ReadModelWatchEndpointId,
            _: ReadModelWatchId,
        ) -> Result<(), ReadModelWatchRegistryError> {
            if self.0.lock().unwrap().fail_register {
                return Err(ReadModelWatchRegistryError::Backend(Box::new(
                    io::Error::other("register failed"),
                )));
            }
            self.0.lock().unwrap().operations.push("stage_registration");
            uow.operations.push(Operation::Register);
            Ok(())
        }

        async fn remove_watch(
            &self,
            uow: &mut TestUow,
            _: ReadModelWatchEndpointId,
            _: ReadModelWatchId,
        ) -> Result<(), ReadModelWatchRegistryError> {
            if self.0.lock().unwrap().fail_remove {
                return Err(ReadModelWatchRegistryError::Backend(Box::new(
                    io::Error::other("remove failed"),
                )));
            }
            uow.operations.push(Operation::Remove);
            Ok(())
        }

        async fn begin_generation(
            &self,
            uow: &mut TestUow,
            _: ReadModelWatchEndpointId,
            _: ReadModelWatchId,
        ) -> Result<ReadModelWatchGeneration, ReadModelWatchRegistryError> {
            let generation = self.0.lock().unwrap().generation.try_next().unwrap();
            uow.operations.push(Operation::Generation(generation));
            Ok(generation)
        }

        async fn register_targets(
            &self,
            uow: &mut TestUow,
            _: ReadModelWatchEndpointId,
            _: ReadModelWatchId,
            generation: ReadModelWatchGeneration,
            targets: &[ReadModelRef],
        ) -> Result<(), ReadModelWatchRegistryError> {
            uow.operations
                .push(Operation::Targets(generation, targets.to_vec()));
            Ok(())
        }

        async fn contains_targets(
            &self,
            _: &mut TestUow,
            _: ReadModelWatchEndpointId,
            _: ReadModelWatchId,
            generation: ReadModelWatchGeneration,
            targets: &[ReadModelRef],
        ) -> Result<bool, ReadModelWatchRegistryError> {
            let state = self.0.lock().unwrap();
            Ok(targets.iter().all(|target| {
                state
                    .targets
                    .get(&generation)
                    .is_some_and(|registered| registered.contains(target))
            }))
        }

        async fn delete_previous_generation_targets(
            &self,
            uow: &mut TestUow,
            _: ReadModelWatchEndpointId,
            _: ReadModelWatchId,
            generation: ReadModelWatchGeneration,
        ) -> Result<(), ReadModelWatchRegistryError> {
            uow.operations.push(Operation::Complete(generation));
            Ok(())
        }
    }

    impl QueryDispatcher for Backend {
        type Uow = TestUow;

        async fn dispatch<H>(
            &self,
            handler: &H,
            context: &RequestContext,
            query: H::Query,
        ) -> Result<H::Output, QueryDispatcherError<H::Error>>
        where
            H: QueryHandler<Uow = TestUow>,
        {
            self.0.lock().unwrap().operations.push("query");
            let mut uow = self.begin().await?;
            handler
                .handle(&mut uow, context, query)
                .await
                .map_err(QueryDispatcherError::Handler)
        }
    }

    struct Endpoint {
        id: ReadModelWatchEndpointId,
        backend: Backend,
        sender: watch::Sender<()>,
    }

    impl ReadModelWatchEndpoint for Endpoint {
        type Receiver = DefaultReadModelWatchReceiver;

        fn is_stop_requested(&self) -> bool {
            false
        }

        fn request_graceful_stop(&self) {}

        fn endpoint_id(&self) -> ReadModelWatchEndpointId {
            self.id
        }

        async fn register_watch(
            &self,
            _: ReadModelWatchId,
        ) -> Result<Self::Receiver, ReadModelWatchEndpointError> {
            self.backend
                .0
                .lock()
                .unwrap()
                .operations
                .push("register_local");
            if self.backend.0.lock().unwrap().fail_receiver {
                return Err(ReadModelWatchEndpointError::Closed);
            }
            Ok(DefaultReadModelWatchReceiver::new(self.sender.subscribe()))
        }

        async fn remove_watch(
            &self,
            _: ReadModelWatchId,
        ) -> Result<(), ReadModelWatchEndpointError> {
            if self.backend.0.lock().unwrap().fail_cleanup {
                return Err(ReadModelWatchEndpointError::StatePoisoned);
            }
            self.backend
                .0
                .lock()
                .unwrap()
                .operations
                .push("remove_local");
            Ok(())
        }

        async fn run_forever(&self) -> Result<(), ReadModelWatchEndpointError> {
            Ok(())
        }
    }

    #[derive(Clone)]
    struct TestQuery;

    impl Query for TestQuery {
        const NAME: QueryName = QueryName::new("watch_test");
    }

    struct Document(Vec<ReadModelRef>);

    impl ReadModelDocument for Document {
        fn read_model_refs(&self) -> Vec<ReadModelRef> {
            self.0.clone()
        }

        fn try_to_resource_document(
            &self,
        ) -> Result<ReadModelResourceDocument, ReadModelDocumentError> {
            panic!("watching must not serialize documents")
        }
    }

    enum Response {
        Document(Vec<ReadModelRef>),
        Fail,
        NotFound,
        Block,
    }

    #[derive(Debug, thiserror::Error)]
    #[error("query failed")]
    struct HandlerError {
        retryable: bool,
    }

    impl crate::Retryability for HandlerError {
        fn is_retryable(&self) -> bool {
            self.retryable
        }
    }

    struct Handler(Mutex<VecDeque<Response>>);

    impl QueryHandler for Handler {
        type Query = TestQuery;
        type Output = Document;
        type Error = HandlerError;
        type Uow = TestUow;

        async fn handle(
            &self,
            _: &mut TestUow,
            _: &RequestContext,
            _: TestQuery,
        ) -> Result<Document, HandlerError> {
            let response = self.0.lock().unwrap().pop_front().expect("queued response");
            match response {
                Response::Document(targets) => Ok(Document(targets)),
                Response::Fail => Err(HandlerError { retryable: true }),
                Response::NotFound => Err(HandlerError { retryable: false }),
                Response::Block => pending().await,
            }
        }
    }

    struct Setup {
        backend: Backend,
        endpoint: Endpoint,
        factory: DefaultReadModelWatchGenerationRegistrarFactory<Backend>,
        handler: Handler,
        context: RequestContext,
    }

    impl Setup {
        fn new(responses: Vec<Response>) -> Self {
            let backend = Backend::default();
            let (sender, _) = watch::channel(());
            Self {
                endpoint: Endpoint {
                    id: ReadModelWatchEndpointId::new(),
                    backend: backend.clone(),
                    sender,
                },
                factory: DefaultReadModelWatchGenerationRegistrarFactory::new(backend.clone()),
                backend,
                handler: Handler(Mutex::new(responses.into())),
                context: RequestContext::new(
                    CorrelationId::from(Uuid::now_v7()),
                    MessageId::new(),
                    Principal::System,
                )
                .unwrap(),
            }
        }

        fn watch(
            &self,
        ) -> DefaultReadModelWatch<
            '_,
            Backend,
            Backend,
            Backend,
            Endpoint,
            DefaultReadModelWatchGenerationRegistrarFactory<Backend>,
            Handler,
        > {
            DefaultReadModelWatch::new(
                DefaultReadModelWatchDependencies {
                    query_dispatcher: &self.backend,
                    uow_factory: &self.backend,
                    registry: &self.backend,
                    endpoint: &self.endpoint,
                    generation_registrar_factory: &self.factory,
                },
                &self.handler,
                &self.context,
                TestQuery,
                ReadModelWatchId::new(),
                DefaultReadModelWatchReceiver::new(self.endpoint.sender.subscribe()),
            )
        }
    }

    fn target(id: &str) -> ReadModelRef {
        ReadModelRef {
            read_model_type: "user".try_into().unwrap(),
            id: ReadModelId::new(id.to_owned()),
        }
    }

    #[tokio::test]
    async fn non_retryable_query_error_closes_watch() {
        for during_refresh in [false, true] {
            let mut responses = Vec::new();
            if during_refresh {
                responses.push(Response::Document(vec![target("old")]));
                responses.push(Response::Document(vec![target("old")]));
            }
            responses.push(Response::NotFound);
            let setup = Setup::new(responses);
            let mut watch = setup.watch();
            if during_refresh {
                watch.next().await.unwrap();
                setup.endpoint.sender.send_replace(());
            }
            assert!(matches!(
                watch.next().await,
                Err(ReadModelWatchError::QueryDispatcher(
                    QueryDispatcherError::Handler(HandlerError { retryable: false })
                ))
            ));
            assert!(matches!(
                watch.next().await.unwrap(),
                ReadModelWatchResult::Closed
            ));
            let state = setup.backend.0.lock().unwrap();
            assert!(state.targets.is_empty());
            assert!(state.operations.ends_with(&["remove_db", "remove_local"]));
        }
    }

    #[tokio::test]
    async fn failed_cleanup_preserves_query_error_and_retries_closing() {
        let setup = Setup::new(vec![Response::NotFound]);
        setup.backend.0.lock().unwrap().fail_remove = true;
        let mut watch = setup.watch();
        assert!(matches!(
            watch.next().await,
            Err(ReadModelWatchError::OperationAndCleanupFailed {
                operation_error: QueryDispatcherError::Handler(HandlerError { retryable: false }),
                ..
            })
        ));
        setup.backend.0.lock().unwrap().fail_remove = false;
        assert!(matches!(
            watch.next().await.unwrap(),
            ReadModelWatchResult::Closed
        ));
    }

    #[tokio::test]
    async fn registers_new_targets_and_requeries_until_covered() {
        let setup = Setup::new(vec![
            Response::Document(vec![target("primary")]),
            Response::Document(vec![target("primary"), target("included")]),
            Response::Document(vec![target("included")]),
        ]);
        let mut watch = setup.watch();
        let ReadModelWatchResult::Document(document) = watch.next().await.unwrap() else {
            panic!("expected document");
        };
        assert_eq!(document.0, vec![target("included")]);
        assert_eq!(
            setup.backend.0.lock().unwrap().operations,
            [
                "begin", "query", "register", "query", "register", "query", "complete"
            ]
        );
    }

    #[tokio::test]
    async fn consumed_notification_is_retried_after_error_and_cancellation() {
        let setup = Setup::new(vec![
            Response::Document(vec![target("old")]),
            Response::Document(vec![target("old")]),
            Response::Fail,
            Response::Block,
            Response::Document(vec![target("new")]),
            Response::Document(vec![target("new")]),
        ]);
        let mut watch = setup.watch();
        watch.next().await.unwrap();
        setup.endpoint.sender.send_replace(());
        assert!(watch.next().await.is_err());
        assert!(
            timeout(Duration::from_millis(10), watch.next())
                .await
                .is_err()
        );
        assert!(
            setup
                .backend
                .0
                .lock()
                .unwrap()
                .targets
                .values()
                .any(|targets| targets.contains(&target("old")))
        );
        let ReadModelWatchResult::Document(document) =
            timeout(Duration::from_secs(1), watch.next())
                .await
                .unwrap()
                .unwrap()
        else {
            panic!("expected document");
        };
        assert_eq!(document.0, vec![target("new")]);
        let state = setup.backend.0.lock().unwrap();
        assert_eq!(state.targets.len(), 1);
        assert!(
            !state
                .targets
                .values()
                .any(|targets| targets.contains(&target("old")))
        );
    }

    #[tokio::test]
    async fn pending_notification_survives_refresh_and_close_is_idempotent() {
        let setup = Setup::new(
            (0..4)
                .map(|_| Response::Document(vec![target("one")]))
                .collect(),
        );
        let mut watch = setup.watch();
        setup.endpoint.sender.send_replace(());
        watch.next().await.unwrap();
        timeout(Duration::from_secs(1), watch.next())
            .await
            .unwrap()
            .unwrap();
        watch.close().await.unwrap();
        watch.close().await.unwrap();
        assert!(matches!(
            watch.next().await.unwrap(),
            ReadModelWatchResult::Closed
        ));
        let state = setup.backend.0.lock().unwrap();
        assert!(state.operations.ends_with(&["remove_db", "remove_local"]));
        assert_eq!(
            state
                .operations
                .iter()
                .filter(|op| **op == "remove_db")
                .count(),
            1
        );
    }

    #[tokio::test]
    async fn failed_close_keeps_local_route_until_database_removal_succeeds() {
        let setup = Setup::new(vec![]);
        let mut watch = setup.watch();
        setup.backend.0.lock().unwrap().fail_remove = true;
        assert!(watch.close().await.is_err());
        assert_eq!(setup.backend.0.lock().unwrap().operations, ["rollback"]);
        setup.backend.0.lock().unwrap().fail_remove = false;
        assert!(matches!(
            watch.next().await.unwrap(),
            ReadModelWatchResult::Closed
        ));
        assert_eq!(
            setup.backend.0.lock().unwrap().operations,
            ["rollback", "remove_db", "remove_local"]
        );
    }

    #[tokio::test]
    async fn watcher_registers_before_returning_and_defers_query_until_next() {
        use crate::read_model::watch::{DefaultReadModelWatcher, ReadModelWatcher};

        let setup = Setup::new(vec![Response::Document(vec![]), Response::Document(vec![])]);
        let watcher = DefaultReadModelWatcher::new(
            setup.backend.clone(),
            setup.backend.clone(),
            setup.backend.clone(),
            setup.endpoint,
            setup.factory,
        );
        let mut first = watcher
            .watch(&setup.handler, &setup.context, TestQuery)
            .await
            .unwrap();
        let mut second = watcher
            .watch(&setup.handler, &setup.context, TestQuery)
            .await
            .unwrap();
        assert_ne!(first.watch_id(), second.watch_id());
        assert_eq!(
            setup.backend.0.lock().unwrap().operations,
            [
                "stage_registration",
                "register_local",
                "register_db",
                "stage_registration",
                "register_local",
                "register_db"
            ]
        );
        assert!(matches!(
            first.next().await.unwrap(),
            ReadModelWatchResult::Document(_)
        ));
        first.close().await.unwrap();
        second.close().await.unwrap();
    }

    #[tokio::test]
    async fn watcher_rolls_back_registration_or_cleans_up_after_commit_failure() {
        use crate::read_model::watch::{
            DefaultReadModelWatcher, ReadModelWatcher, ReadModelWatcherError,
        };

        for failure in ["begin", "register", "receiver", "commit", "cleanup"] {
            let setup = Setup::new(vec![]);
            {
                let mut state = setup.backend.0.lock().unwrap();
                state.fail_begin = failure == "begin";
                state.fail_register = failure == "register";
                state.fail_receiver = failure == "receiver";
                state.fail_commit = failure == "commit" || failure == "cleanup";
                state.fail_cleanup = failure == "cleanup";
            }
            let watcher = DefaultReadModelWatcher::new(
                setup.backend.clone(),
                setup.backend.clone(),
                setup.backend.clone(),
                setup.endpoint,
                setup.factory,
            );
            let result = watcher
                .watch(&setup.handler, &setup.context, TestQuery)
                .await;
            match failure {
                "begin" => assert!(matches!(
                    result,
                    Err(ReadModelWatcherError::UnitOfWorkFactory(_))
                )),
                "register" => assert!(matches!(result, Err(ReadModelWatcherError::Registry(_)))),
                "receiver" => assert!(matches!(result, Err(ReadModelWatcherError::Endpoint(_)))),
                "commit" => assert!(matches!(result, Err(ReadModelWatcherError::UnitOfWork(_)))),
                "cleanup" => assert!(matches!(
                    result,
                    Err(ReadModelWatcherError::OperationAndCleanupFailed { .. })
                )),
                _ => unreachable!(),
            }
            assert!(watcher.endpoint().sender.is_closed());
            let state = setup.backend.0.lock().unwrap();
            let expected: &[&str] = match failure {
                "begin" => &[],
                "register" => &["rollback"],
                "receiver" => &["stage_registration", "register_local", "rollback"],
                "commit" => &["stage_registration", "register_local", "remove_local"],
                "cleanup" => &["stage_registration", "register_local"],
                _ => unreachable!(),
            };
            assert_eq!(state.operations, expected);
        }
    }
}
