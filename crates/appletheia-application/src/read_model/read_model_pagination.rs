use serde::Serialize;

use super::ReadModelCursor;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReadModelPagination {
    pub start_cursor: Option<ReadModelCursor>,
    pub end_cursor: Option<ReadModelCursor>,
    pub has_before: bool,
    pub has_after: bool,
}
