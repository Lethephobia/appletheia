use appletheia::aggregate_state;
use appletheia::unique_constraints;

use super::{OrganizationId, OrganizationName, OrganizationStateError};

/// Stores the materialized state of an `Organization` aggregate.
#[aggregate_state(error = OrganizationStateError)]
#[unique_constraints()]
pub struct OrganizationState {
    pub(super) id: OrganizationId,
    pub(super) name: OrganizationName,
}

impl OrganizationState {
    /// Creates a new organization state.
    pub(super) fn new(id: OrganizationId, name: OrganizationName) -> Self {
        Self { id, name }
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateState;

    use super::{OrganizationId, OrganizationName, OrganizationState};

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = OrganizationId::new();
        let state = OrganizationState::new(
            id,
            OrganizationName::try_from("Acme Labs").expect("name should be valid"),
        );

        assert_eq!(state.id(), id);
    }
}
