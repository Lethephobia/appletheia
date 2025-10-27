use thiserror::Error;

use crate::aggregate::Aggregate;
use crate::event::event_history_reader_error::EventHistoryReaderError;

#[derive(Debug, Error)]
pub enum RepositoryError<A: Aggregate> {
    #[error("aggregate error: {0}")]
    AggregateError(#[source] A::Error),

    #[error("event history reader error: {0}")]
    EventHistoryReaderError(#[source] EventHistoryReaderError),
}
