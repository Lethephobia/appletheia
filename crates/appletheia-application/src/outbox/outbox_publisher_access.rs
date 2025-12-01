use super::OutboxPublisher;

pub trait OutboxPublisherAccess {
    type OutboxPublisher: OutboxPublisher;

    fn outbox_publisher(&self) -> &Self::OutboxPublisher;
}
