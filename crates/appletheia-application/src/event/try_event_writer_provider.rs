use appletheia_domain::Aggregate;
use std::error::Error;

use super::EventWriter;

pub trait TryEventWriterProvider<A: Aggregate> {
    type Error: Error + Send + Sync + 'static;
    type EventWriter<'c>: EventWriter<A, Error = Self::Error>
    where
        Self: 'c;

    fn try_event_writer(&mut self) -> Result<Self::EventWriter<'_>, Self::Error>;
}
