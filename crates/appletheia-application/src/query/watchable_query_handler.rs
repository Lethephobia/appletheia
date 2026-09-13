use crate::read_model::watch::ReadModelWatchSelector;

use super::QueryHandler;

/// Declares prospective dependencies that can cause a retained query result to change.
pub trait WatchableQueryHandler: QueryHandler {
    /// Returns dependencies that are not necessarily present in the current snapshot.
    fn watch_dependencies(
        &self,
        query: &Self::Query,
    ) -> Result<Vec<ReadModelWatchSelector>, Self::Error>;
}
