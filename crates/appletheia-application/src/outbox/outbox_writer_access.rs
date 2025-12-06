use super::OutboxWriter;

pub trait OutboxWriterAccess {
    type Writer: OutboxWriter;

    fn outbox_writer(&self) -> &Self::Writer;
}
