use super::QueryConsistency;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct QueryOptions {
    pub consistency: QueryConsistency,
}
