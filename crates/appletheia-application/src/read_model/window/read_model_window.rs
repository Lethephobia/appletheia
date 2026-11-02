use serde::Serialize;

use super::ReadModelWindowCursor;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReadModelWindow {
    pub start_cursor: Option<ReadModelWindowCursor>,
    pub end_cursor: Option<ReadModelWindowCursor>,
    pub has_before: bool,
    pub has_after: bool,
}
