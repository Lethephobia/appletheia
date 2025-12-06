use appletheia_domain::Aggregate;

use super::EventReader;

pub trait EventReaderAccess<A: Aggregate> {
    type Reader: EventReader<A>;

    fn event_reader(&self) -> &Self::Reader;
}
