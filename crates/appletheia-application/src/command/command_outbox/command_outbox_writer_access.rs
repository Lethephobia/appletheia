use super::CommandOutboxWriter;

pub trait CommandOutboxWriterAccess {
    type Writer: CommandOutboxWriter;

    fn command_outbox_writer(&self) -> &Self::Writer;
}
