use std::fmt::Debug;
use thiserror::Error;

use appletheia_domain::Aggregate;

use crate::event::EventReaderError;
use crate::snapshot::SnapshotReaderError;

#[derive(Debug, Error)]
pub enum RepositoryError<A: Aggregate> {
    #[error("aggregate error: {0}")]
    Aggregate(#[source] A::Error),

    #[error("event reader error: {0}")]
    EventReader(#[from] EventReaderError),

    #[error("snapshot reader error: {0}")]
    SnapshotReader(#[from] SnapshotReaderError),
}
