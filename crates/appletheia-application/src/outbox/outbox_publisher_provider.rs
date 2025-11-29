use super::OutboxPublisher;

pub trait OutboxPublisherProvider {
    type OutboxPublisher: OutboxPublisher;

    fn outbox_publisher(&self) -> &Self::OutboxPublisher;
}
