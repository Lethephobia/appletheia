use std::future::Future;
use std::pin::Pin;

use super::{ReadModelWatchRefresh, ReadModelWatchRefreshError, ReadModelWatchRefreshRequest};

pub type ReadModelWatchRefreshFuture<'a> =
    Pin<Box<dyn Future<Output = Result<ReadModelWatchRefresh, ReadModelWatchRefreshError>> + 'a>>;

/// Reauthorizes and reruns one retained query on the server.
pub trait ReadModelWatchSubscriptionExecutor: Send + Sync + 'static {
    fn refresh(&self, request: ReadModelWatchRefreshRequest) -> ReadModelWatchRefreshFuture<'_>;
}
