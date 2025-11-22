use appletheia_domain::{Aggregate, Event};
use std::error::Error;

use crate::request_context::RequestContextAccess;

#[allow(async_fn_in_trait)]
pub trait EventWriter<A: Aggregate>: RequestContextAccess {
    type Error: Error + Send + Sync + 'static;

    async fn write_events_and_outbox(
        &mut self,
        events: &[Event<A::Id, A::EventPayload>],
    ) -> Result<(), Self::Error>;
}
