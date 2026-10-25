use serde::{Deserialize, Serialize};

use crate::{
    messaging::{OrderingKey, PublishableMessage},
    read_model::ReadModelChangeId,
};

use super::{ReadModelWatchNotification, ReadModelWatchNotificationId, ReadModelWatchNotifiedAt};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReadModelWatchNotificationEnvelope {
    pub notification_id: ReadModelWatchNotificationId,
    pub source_change_id: ReadModelChangeId,
    pub notified_at: ReadModelWatchNotifiedAt,
    pub notification: ReadModelWatchNotification,
}

impl ReadModelWatchNotificationEnvelope {
    pub fn new(
        source_change_id: ReadModelChangeId,
        notification: ReadModelWatchNotification,
    ) -> Self {
        Self {
            notification_id: ReadModelWatchNotificationId::new(),
            source_change_id,
            notified_at: ReadModelWatchNotifiedAt::now(),
            notification,
        }
    }
}

impl PublishableMessage for ReadModelWatchNotificationEnvelope {
    fn ordering_key(&self) -> OrderingKey {
        OrderingKey::from(self.notification.endpoint_id)
    }
}
