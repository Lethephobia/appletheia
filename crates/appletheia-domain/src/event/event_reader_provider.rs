use crate::aggregate::Aggregate;

use super::EventReader;

pub trait EventReaderProvider<A: Aggregate> {
    type EventReader<'c>: EventReader<A>
    where
        Self: 'c;

    fn event_reader(&mut self) -> Self::EventReader<'_>;
}
