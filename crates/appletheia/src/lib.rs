#[cfg(feature = "application")]
pub mod application {
    pub use appletheia_application::*;
}

#[cfg(feature = "domain")]
pub mod domain {
    pub use appletheia_domain::*;
}

#[cfg(feature = "infrastructure")]
pub mod infrastructure {
    pub use appletheia_infrastructure::*;
}

#[cfg(feature = "macros")]
pub use appletheia_macros::{
    Aggregate, AggregateId, AggregateState, Command, EventPayload, aggregate, aggregate_id,
    aggregate_state, command, event_payload, unique_constraints,
};
