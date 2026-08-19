use std::future::Future;

use super::{ReadModelWatchDeliveryError, ReadModelWatchEvent, ReadModelWatchSessionId};

/// Delivers one complete-snapshot protocol event to an established client connection.
pub trait ReadModelWatchDelivery: Send + Sync + 'static {
    fn deliver(
        &self,
        session_id: &ReadModelWatchSessionId,
        event: &ReadModelWatchEvent,
    ) -> impl Future<Output = Result<(), ReadModelWatchDeliveryError>> + Send;
}
