use thiserror::Error;

use super::{EventOutboxAttemptCountError, EventOutboxLifecycle, EventOutboxState};

#[derive(Debug, Error)]
pub enum EventOutboxError {
    #[error("outbox attempt count error: {0}")]
    AttemptCount(#[from] EventOutboxAttemptCountError),

    #[error("cannot acknowledge a dead-lettered outbox in lifecycle {0:?}")]
    AckOnDeadLettered(EventOutboxLifecycle),

    #[error("cannot negatively acknowledge a dead-lettered outbox in lifecycle {0:?}")]
    NackOnDeadLettered(EventOutboxLifecycle),

    #[error("cannot extend lease for a dead-lettered outbox in lifecycle {0:?}")]
    ExtendLeaseOnDeadLettered(EventOutboxLifecycle),

    #[error("cannot acquire lease for a dead-lettered outbox in lifecycle {0:?}")]
    AcquireLeaseOnDeadLettered(EventOutboxLifecycle),

    #[error("extend_lease is only valid in leased state, got {0:?}")]
    ExtendLeaseOnNonLeased(EventOutboxState),

    #[error("acquire_lease is only valid in pending state, got {0:?}")]
    AcquireLeaseOnNonPending(EventOutboxState),
}
