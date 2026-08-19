use crate::query::{QueryDispatcher, QueryHandler, QueryOptions, WatchableQueryHandler};
use crate::read_model::{ReadModel, ReadModelDependency};
use crate::request_context::RequestContext;

use super::default_query_handler_read_model_watch_executor::map_dispatch_error;
use super::{
    ReadModelListChunkDescriptor, ReadModelWatchFailure, ReadModelWatchRefresh,
    ReadModelWatchRefreshError, ReadModelWatchRefreshFuture, ReadModelWatchRefreshRequest,
    ReadModelWatchRefreshValue, ReadModelWatchSubscriptionExecutor, SerializedReadModelListChunk,
};

/// Reruns one contiguous active list window and splits the result into client chunks.
pub struct DefaultQueryHandlerReadModelListWatchExecutor<H, D, P, S>
where
    H: QueryHandler,
{
    handler: H,
    dispatcher: D,
    request_context: RequestContext,
    base_query: H::Query,
    options: QueryOptions,
    plan_window: P,
    split_window: S,
}

impl<H, D, P, S> DefaultQueryHandlerReadModelListWatchExecutor<H, D, P, S>
where
    H: QueryHandler,
{
    pub fn new(
        handler: H,
        dispatcher: D,
        request_context: RequestContext,
        base_query: H::Query,
        options: QueryOptions,
        plan_window: P,
        split_window: S,
    ) -> Self {
        Self {
            handler,
            dispatcher,
            request_context,
            base_query,
            options,
            plan_window,
            split_window,
        }
    }
}

impl<H, D, P, S> DefaultQueryHandlerReadModelListWatchExecutor<H, D, P, S>
where
    H: WatchableQueryHandler,
{
    /// Resolves dependencies that can add values to the active list window.
    pub fn prospective_dependencies(&self) -> Result<Vec<ReadModelDependency>, H::Error> {
        self.handler.watch_dependencies(&self.base_query)
    }
}

impl<H, D, P, S> ReadModelWatchSubscriptionExecutor
    for DefaultQueryHandlerReadModelListWatchExecutor<H, D, P, S>
where
    H: QueryHandler + 'static,
    H::Query: Sync,
    D: QueryDispatcher<Uow = H::Uow> + 'static,
    P: Fn(
            &H::Query,
            &[ReadModelListChunkDescriptor],
        ) -> Result<H::Query, ReadModelWatchRefreshError>
        + Send
        + Sync
        + 'static,
    S: Fn(
            &[ReadModelListChunkDescriptor],
            H::Output,
        ) -> Result<Vec<SerializedReadModelListChunk>, ReadModelWatchRefreshError>
        + Send
        + Sync
        + 'static,
{
    fn refresh(&self, request: ReadModelWatchRefreshRequest) -> ReadModelWatchRefreshFuture<'_> {
        Box::pin(async move {
            let ReadModelWatchRefreshRequest::List { active_chunks } = request else {
                return Err(ReadModelWatchRefreshError::Failed(ReadModelWatchFailure {
                    code: "list_executor_received_snapshot_request".to_owned(),
                    retryable: false,
                }));
            };
            let query = (self.plan_window)(&self.base_query, &active_chunks)?;
            let output = self
                .dispatcher
                .dispatch(
                    &self.handler,
                    &self.request_context,
                    query,
                    self.options.clone(),
                )
                .await
                .map_err(map_dispatch_error)?;
            let partitions = output.partitions().map_err(|_| {
                ReadModelWatchRefreshError::Failed(ReadModelWatchFailure {
                    code: "list_partition_serialization_failed".to_owned(),
                    retryable: false,
                })
            })?;
            let chunks = (self.split_window)(&active_chunks, output)?;

            Ok(ReadModelWatchRefresh {
                value: ReadModelWatchRefreshValue::List(chunks),
                materialized_dependencies: partitions
                    .into_iter()
                    .map(ReadModelDependency::Partition)
                    .collect(),
            })
        })
    }
}
