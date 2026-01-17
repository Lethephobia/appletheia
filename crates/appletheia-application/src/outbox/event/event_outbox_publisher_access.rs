use super::EventOutboxPublisher;

pub trait EventOutboxPublisherAccess {
    type EventOutboxPublisher: EventOutboxPublisher;

    fn outbox_publisher(&self) -> &Self::EventOutboxPublisher;
}
