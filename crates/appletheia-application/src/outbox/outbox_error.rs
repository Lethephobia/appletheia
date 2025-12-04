use thiserror::Error;

use super::{OutboxAttemptCountError, OutboxLifecycle, OutboxState};

#[derive(Debug, Error)]
pub enum OutboxError {
    #[error("outbox attempt count error: {0}")]
    AttemptCount(#[from] OutboxAttemptCountError),

    #[error("cannot acknowledge a dead-lettered outbox in lifecycle {0:?}")]
    AckOnDeadLettered(OutboxLifecycle),

    #[error("cannot negatively acknowledge a dead-lettered outbox in lifecycle {0:?}")]
    NackOnDeadLettered(OutboxLifecycle),

    #[error("cannot extend lease for a dead-lettered outbox in lifecycle {0:?}")]
    ExtendLeaseOnDeadLettered(OutboxLifecycle),

    #[error("cannot acquire lease for a dead-lettered outbox in lifecycle {0:?}")]
    AcquireLeaseOnDeadLettered(OutboxLifecycle),

    #[error("extend_lease is only valid in leased state, got {0:?}")]
    ExtendLeaseOnNonLeased(OutboxState),

    #[error("acquire_lease is only valid in pending state, got {0:?}")]
    AcquireLeaseOnNonPending(OutboxState),
}
