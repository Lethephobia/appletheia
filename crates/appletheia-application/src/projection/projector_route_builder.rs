use super::ProjectorEventRouteHandlerBuilder;
use appletheia_domain::{Aggregate, EventName};

pub struct ProjectorRouteBuilder;

impl ProjectorRouteBuilder {
    pub fn on<A: Aggregate>(event_name: EventName) -> ProjectorEventRouteHandlerBuilder<A> {
        ProjectorEventRouteHandlerBuilder::new(event_name)
    }
}
