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
    Aggregate, AggregateId, AggregateState, EventPayload, aggregate, aggregate_id, aggregate_state,
    event_payload,
};
