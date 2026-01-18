use crate::outbox::OutboxPublishResult;

use super::CommandOutboxId;

pub type CommandOutboxPublishResult = OutboxPublishResult<CommandOutboxId>;
