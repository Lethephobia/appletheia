use std::future::Future;

use super::{ReadModelWatchDeliveryError, ReadModelWatchRoute, ReadModelWatchSessionId};

/// Delivers one routed change to an already established client connection.
pub trait ReadModelWatchDelivery: Send + Sync + 'static {
    fn deliver(
        &self,
        session_id: &ReadModelWatchSessionId,
        route: &ReadModelWatchRoute,
    ) -> impl Future<Output = Result<(), ReadModelWatchDeliveryError>> + Send;
}
