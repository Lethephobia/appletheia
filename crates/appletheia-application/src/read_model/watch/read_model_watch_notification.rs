use serde::{Deserialize, Serialize};

use crate::read_model::ReadModelRef;

use super::ReadModelWatchEndpointId;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReadModelWatchNotification {
    pub endpoint_id: ReadModelWatchEndpointId,
    pub target: ReadModelRef,
}

impl ReadModelWatchNotification {
    pub fn new(endpoint_id: ReadModelWatchEndpointId, target: ReadModelRef) -> Self {
        Self {
            endpoint_id,
            target,
        }
    }
}
