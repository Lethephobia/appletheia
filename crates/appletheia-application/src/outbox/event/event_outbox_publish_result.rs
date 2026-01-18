use crate::outbox::OutboxPublishResult;

use super::EventOutboxId;

pub type EventOutboxPublishResult = OutboxPublishResult<EventOutboxId>;
