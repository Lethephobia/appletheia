use std::error::Error;

use crate::event::EventPayload;

pub trait AggregateApply<P, E>
where
    P: EventPayload,
    E: Error + Send + Sync + 'static,
{
    fn apply(&mut self, payload: &P) -> Result<(), E>;
}
