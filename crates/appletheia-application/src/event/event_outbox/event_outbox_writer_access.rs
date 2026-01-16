use super::EventOutboxWriter;

pub trait EventOutboxWriterAccess {
    type Writer: EventOutboxWriter;

    fn outbox_writer(&self) -> &Self::Writer;
}
