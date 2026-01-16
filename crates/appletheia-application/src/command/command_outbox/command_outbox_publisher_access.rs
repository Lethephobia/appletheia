use super::CommandOutboxPublisher;

pub trait CommandOutboxPublisherAccess {
    type CommandOutboxPublisher: CommandOutboxPublisher;

    fn command_outbox_publisher(&self) -> &Self::CommandOutboxPublisher;
}
