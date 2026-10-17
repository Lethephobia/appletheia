use std::error::Error as StdError;

use thiserror::Error;

use crate::query::QueryDispatcherError;

#[derive(Debug, Error)]
pub enum ReadModelWatchError<HE>
where
    HE: StdError + Send + Sync + 'static,
{
    #[error(transparent)]
    QueryDispatcher(#[from] QueryDispatcherError<HE>),

    #[error("read model watch subscription backend error: {0}")]
    Backend(#[source] Box<dyn StdError + Send + Sync + 'static>),
}
