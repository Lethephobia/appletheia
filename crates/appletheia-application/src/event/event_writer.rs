use appletheia_domain::{Aggregate, Event};

#[allow(async_fn_in_trait)]
pub trait EventWriter<A: Aggregate> {
    type Error;

    async fn write_events_and_outbox(
        &mut self,
        events: &[Event<A::Id, A::EventPayload>],
    ) -> Result<(), Self::Error>;
}
