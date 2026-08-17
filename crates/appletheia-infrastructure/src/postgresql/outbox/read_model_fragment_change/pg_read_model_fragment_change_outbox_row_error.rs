use thiserror::Error;

use appletheia_application::event::{AggregateTypeOwnedError, EventSequenceError};
use appletheia_application::outbox::{OutboxAttemptCountError, OutboxRelayInstanceError};
use appletheia_application::projection::ProjectorNameOwnedError;
use appletheia_application::read_model::{
    ReadModelFragmentChangeEnvelopeError, ReadModelFragmentChangeIdError, SerializedPartitionError,
};
use appletheia_domain::EventIdError;

/// Reports invalid state or values loaded from the fragment-change outbox.
#[derive(Debug, Error)]
pub enum PgReadModelFragmentChangeOutboxRowError {
    #[error("fragment change id error: {0}")]
    ChangeId(#[from] ReadModelFragmentChangeIdError),
    #[error("source partition error: {0}")]
    SourcePartition(#[from] SerializedPartitionError),
    #[error("source projector name error: {0}")]
    SourceProjectorName(#[from] ProjectorNameOwnedError),
    #[error("source event sequence error: {0}")]
    SourceEventSequence(#[from] EventSequenceError),
    #[error("source event id error: {0}")]
    SourceEventId(#[from] EventIdError),
    #[error("source aggregate type error: {0}")]
    SourceAggregateType(#[from] AggregateTypeOwnedError),
    #[error("attempt count error: {0}")]
    AttemptCount(#[from] OutboxAttemptCountError),
    #[error("lease owner error: {0}")]
    LeaseOwner(#[from] OutboxRelayInstanceError),
    #[error("json mapping error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("fragment change envelope error: {0}")]
    Envelope(#[from] ReadModelFragmentChangeEnvelopeError),
    #[error("outbox row contained inconsistent lease state")]
    InconsistentLeaseState,
}
