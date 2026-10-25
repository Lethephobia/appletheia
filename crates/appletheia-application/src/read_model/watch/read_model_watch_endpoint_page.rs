use super::ReadModelWatchEndpointId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadModelWatchEndpointPage {
    pub endpoint_ids: Vec<ReadModelWatchEndpointId>,
    pub next_cursor: Option<ReadModelWatchEndpointId>,
}

impl ReadModelWatchEndpointPage {
    pub fn new(
        endpoint_ids: Vec<ReadModelWatchEndpointId>,
        next_cursor: Option<ReadModelWatchEndpointId>,
    ) -> Self {
        Self {
            endpoint_ids,
            next_cursor,
        }
    }

    pub fn is_last(&self) -> bool {
        self.next_cursor.is_none()
    }
}
