use serde::{Deserialize, Serialize};

use super::ReadModelWindowSortDirection;

/// Defines the stable ordering of a cursor-paginated query.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ReadModelWindowSort<K> {
    /// Selects the application-defined ordering fields.
    pub key: K,
    /// Selects ascending or descending traversal of the ordering.
    pub direction: ReadModelWindowSortDirection,
}
