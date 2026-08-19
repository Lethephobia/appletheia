use crate::read_model::ReadModelDependency;

use super::ReadModelWatchRefreshValue;

/// Contains a refreshed value and the Fragment dependencies it materialized.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelWatchRefresh {
    pub value: ReadModelWatchRefreshValue,
    pub materialized_dependencies: Vec<ReadModelDependency>,
}
