use super::QueryConsistency;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct QueryOptions {
    pub consistency: QueryConsistency,
}
