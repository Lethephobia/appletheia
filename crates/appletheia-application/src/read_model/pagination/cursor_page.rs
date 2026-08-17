use serde::{Deserialize, Serialize};

use super::PageSize;

/// Describes one forward page in a cursor-paginated query.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CursorPage<C> {
    /// Starts the page after this cursor, or at the beginning when absent.
    pub after: Option<C>,
    /// Limits the number of items returned by the query.
    pub limit: PageSize,
}
