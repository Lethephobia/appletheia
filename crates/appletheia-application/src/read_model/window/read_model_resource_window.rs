use serde::Serialize;

use super::ReadModelResourceWindowCursor;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReadModelResourceWindow {
    pub start_cursor: Option<ReadModelResourceWindowCursor>,
    pub end_cursor: Option<ReadModelResourceWindowCursor>,
    pub has_before: bool,
    pub has_after: bool,
}
