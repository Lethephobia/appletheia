use serde::{Deserialize, Serialize};

/// Describes how far a client has materialized a cursor-paginated list.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ReadModelListCoverage<C> {
    /// Indicates that the client has not materialized any list items.
    Empty,
    /// Covers the ordered range from the beginning through the supplied cursor.
    Through { cursor: C },
    /// Covers the complete result set for the query.
    Complete,
}
