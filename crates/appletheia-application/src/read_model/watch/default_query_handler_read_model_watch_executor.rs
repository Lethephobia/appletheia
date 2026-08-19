use crate::authorization::AuthorizerError;
use crate::query::{
    QueryDispatcher, QueryDispatcherError, QueryHandler, QueryOptions, WatchableQueryHandler,
};
use crate::read_model::{ReadModel, ReadModelDependency};
use crate::request_context::RequestContext;

use super::{
    ReadModelWatchCloseReason, ReadModelWatchFailure, ReadModelWatchRefresh,
    ReadModelWatchRefreshError, ReadModelWatchRefreshFuture, ReadModelWatchRefreshRequest,
    ReadModelWatchRefreshValue, ReadModelWatchSubscriptionExecutor, SerializedReadModelSnapshot,
};

/// Retains a query and reruns its handler through the ordinary query-dispatch path.
pub struct DefaultQueryHandlerReadModelWatchExecutor<H, D>
where
    H: QueryHandler,
{
    handler: H,
    dispatcher: D,
    request_context: RequestContext,
    query: H::Query,
    options: QueryOptions,
}

impl<H, D> DefaultQueryHandlerReadModelWatchExecutor<H, D>
where
    H: QueryHandler,
{
    pub fn new(
        handler: H,
        dispatcher: D,
        request_context: RequestContext,
        query: H::Query,
        options: QueryOptions,
    ) -> Self {
        Self {
            handler,
            dispatcher,
            request_context,
            query,
            options,
        }
    }
}

impl<H, D> DefaultQueryHandlerReadModelWatchExecutor<H, D>
where
    H: WatchableQueryHandler,
{
    /// Resolves dependencies that can materialize even when absent from the current snapshot.
    pub fn prospective_dependencies(&self) -> Result<Vec<ReadModelDependency>, H::Error> {
        self.handler.watch_dependencies(&self.query)
    }
}

impl<H, D> ReadModelWatchSubscriptionExecutor for DefaultQueryHandlerReadModelWatchExecutor<H, D>
where
    H: QueryHandler + 'static,
    H::Query: Clone + Sync,
    D: QueryDispatcher<Uow = H::Uow> + 'static,
{
    fn refresh(&self, request: ReadModelWatchRefreshRequest) -> ReadModelWatchRefreshFuture<'_> {
        Box::pin(async move {
            if !matches!(request, ReadModelWatchRefreshRequest::Snapshot) {
                return Err(ReadModelWatchRefreshError::Failed(ReadModelWatchFailure {
                    code: "snapshot_executor_received_list_request".to_owned(),
                    retryable: false,
                }));
            }

            let output = self
                .dispatcher
                .dispatch(
                    &self.handler,
                    &self.request_context,
                    self.query.clone(),
                    self.options.clone(),
                )
                .await
                .map_err(map_dispatch_error)?;
            let partitions = output.partitions().map_err(|_| {
                ReadModelWatchRefreshError::Failed(ReadModelWatchFailure {
                    code: "snapshot_partition_serialization_failed".to_owned(),
                    retryable: false,
                })
            })?;
            let serialized = serde_json::to_value(output).map_err(|_| {
                ReadModelWatchRefreshError::Failed(ReadModelWatchFailure {
                    code: "snapshot_serialization_failed".to_owned(),
                    retryable: false,
                })
            })?;

            Ok(ReadModelWatchRefresh {
                value: ReadModelWatchRefreshValue::Snapshot(SerializedReadModelSnapshot::from(
                    serialized,
                )),
                materialized_dependencies: partitions
                    .into_iter()
                    .map(ReadModelDependency::Partition)
                    .collect(),
            })
        })
    }
}

pub(super) fn map_dispatch_error<HE>(error: QueryDispatcherError<HE>) -> ReadModelWatchRefreshError
where
    HE: std::error::Error + Send + Sync + 'static,
{
    match error {
        QueryDispatcherError::Authorizer(
            AuthorizerError::PrincipalUnavailable | AuthorizerError::Forbidden,
        ) => ReadModelWatchRefreshError::Closed(ReadModelWatchCloseReason::AuthorizationDenied),
        QueryDispatcherError::Authorizer(AuthorizerError::Backend(_)) => {
            retryable_failure("authorization_backend_unavailable")
        }
        QueryDispatcherError::UnitOfWorkFactory(_) => {
            retryable_failure("unit_of_work_creation_failed")
        }
        QueryDispatcherError::UnitOfWork(_) => retryable_failure("unit_of_work_failed"),
        QueryDispatcherError::ProjectionConsistency(_) => {
            retryable_failure("projection_consistency_wait_failed")
        }
        QueryDispatcherError::Handler(_) => retryable_failure("query_handler_failed"),
    }
}

fn retryable_failure(code: &str) -> ReadModelWatchRefreshError {
    ReadModelWatchRefreshError::Failed(ReadModelWatchFailure {
        code: code.to_owned(),
        retryable: true,
    })
}
