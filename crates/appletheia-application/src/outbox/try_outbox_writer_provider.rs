use std::error::Error;

use super::OutboxWriter;

pub trait TryOutboxWriterProvider {
    type Error: Error + Send + Sync + 'static;
    type OutboxWriter<'c>: OutboxWriter
    where
        Self: 'c;

    fn try_outbox_writer(&mut self) -> Result<Self::OutboxWriter<'_>, Self::Error>;
}
