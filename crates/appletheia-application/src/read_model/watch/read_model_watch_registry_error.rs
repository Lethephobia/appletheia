use thiserror::Error;

use super::{
    ReadModelWatchDeliveryError, ReadModelWatchLimitsError, ReadModelWatchSessionId,
    ReadModelWatchSubscriptionId,
};

/// Reports a failure to maintain or deliver a watch subscription.
#[derive(Debug, Error)]
pub enum ReadModelWatchRegistryError {
    #[error(transparent)]
    InvalidLimits(#[from] ReadModelWatchLimitsError),

    #[error("read-model watch session was not found: {0}")]
    SessionNotFound(ReadModelWatchSessionId),

    #[error("the session reached its subscription limit")]
    SubscriptionLimitExceeded,

    #[error("read-model watch subscription was not found: {0}")]
    SubscriptionNotFound(ReadModelWatchSubscriptionId),

    #[error("the subscription is not a list subscription")]
    NotListSubscription,

    #[error("the list subscription reached its active chunk limit")]
    ActiveChunkLimitExceeded,

    #[error("the subscription revision overflowed")]
    RevisionOverflow,

    #[error("the read-model refresh scheduler is closed")]
    RefreshSchedulerClosed,

    #[error("the read-model delivery backpressure limit was exceeded")]
    DeliveryBackpressureExceeded,

    #[error(transparent)]
    Delivery(#[from] ReadModelWatchDeliveryError),
}
