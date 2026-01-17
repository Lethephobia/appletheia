use super::CommandOutboxWriter;

pub trait CommandOutboxWriterAccess {
    type Writer: CommandOutboxWriter;

    fn outbox_writer(&self) -> &Self::Writer;
}
