use super::{QueryConsistency, ReadModelWatchOptions};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct QueryOptions {
    pub consistency: QueryConsistency,
    pub watch: Option<ReadModelWatchOptions>,
}
