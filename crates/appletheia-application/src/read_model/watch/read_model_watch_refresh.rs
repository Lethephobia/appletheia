use super::{ReadModelWatchRefreshValue, ReadModelWatchSelector};

/// Contains a refreshed value and the Fragment dependencies it materialized.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelWatchRefresh {
    pub value: ReadModelWatchRefreshValue,
    pub materialized_dependencies: Vec<ReadModelWatchSelector>,
}
