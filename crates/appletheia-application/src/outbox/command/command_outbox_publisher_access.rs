use super::CommandOutboxPublisher;

pub trait CommandOutboxPublisherAccess {
    type Publisher: CommandOutboxPublisher;

    fn outbox_publisher(&self) -> &Self::Publisher;
}
