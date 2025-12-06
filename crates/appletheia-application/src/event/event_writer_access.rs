use appletheia_domain::Aggregate;

use super::EventWriter;

pub trait EventWriterAccess<A: Aggregate> {
    type Writer: EventWriter<A>;

    fn event_writer(&self) -> &Self::Writer;
}
