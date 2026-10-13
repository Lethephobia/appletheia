use super::{ReadModelChange, ReadModelChangeId, ReadModelChangedAt};
use crate::messaging::{OrderingKey, PublishableMessage};
use appletheia_domain::EventId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReadModelChangeEnvelope {
    pub change_id: ReadModelChangeId,
    pub source_event_id: EventId,
    pub changed_at: ReadModelChangedAt,
    pub change: ReadModelChange,
}

impl ReadModelChangeEnvelope {
    pub fn new(source_event_id: EventId, change: ReadModelChange) -> Self {
        Self {
            change_id: ReadModelChangeId::new(),
            source_event_id,
            changed_at: ReadModelChangedAt::now(),
            change,
        }
    }
}

impl PublishableMessage for ReadModelChangeEnvelope {
    fn ordering_key(&self) -> OrderingKey {
        let read_model_ref = self.change.read_model_ref();
        OrderingKey::from(read_model_ref)
    }
}
