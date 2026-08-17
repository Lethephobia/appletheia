use thiserror::Error;

use super::{DefaultReadModelWatchChangeRouterError, ReadModelWatchDeliveryError};

/// Reports a failure while routing or delivering one session-specific change.
#[derive(Debug, Error)]
pub enum ReadModelWatchDispatchError {
    #[error(transparent)]
    Route(#[from] DefaultReadModelWatchChangeRouterError),

    #[error(transparent)]
    Delivery(#[from] ReadModelWatchDeliveryError),
}
