use super::ReadModelWatchId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelWatchPage {
    pub watch_ids: Vec<ReadModelWatchId>,
    pub next_cursor: Option<ReadModelWatchId>,
}

impl ReadModelWatchPage {
    pub fn new(watch_ids: Vec<ReadModelWatchId>, next_cursor: Option<ReadModelWatchId>) -> Self {
        Self {
            watch_ids,
            next_cursor,
        }
    }

    pub fn is_last(&self) -> bool {
        self.next_cursor.is_none()
    }
}
