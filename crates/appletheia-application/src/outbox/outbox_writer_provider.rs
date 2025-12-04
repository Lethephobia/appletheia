use super::OutboxWriter;

pub trait OutboxWriterProvider {
    type OutboxWriter<'c>: OutboxWriter
    where
        Self: 'c;

    fn outbox_writer(&mut self) -> Self::OutboxWriter<'_>;
}
