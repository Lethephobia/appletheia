use appletheia::aggregate_state;
use appletheia::domain::{AggregateId, UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::unique_constraints;

use crate::{OrganizationId, UserId};

use super::{
    OrganizationMembershipId, OrganizationMembershipStateError, OrganizationMembershipStatus,
};

/// Stores the materialized state of an `OrganizationMembership` aggregate.
#[aggregate_state(error = OrganizationMembershipStateError)]
#[unique_constraints(entry(key = "organization_user", values = organization_user_values))]
pub struct OrganizationMembershipState {
    pub(super) id: OrganizationMembershipId,
    pub(super) status: OrganizationMembershipStatus,
    pub(super) organization_id: OrganizationId,
    pub(super) user_id: UserId,
}

impl OrganizationMembershipState {
    /// Creates a new organization membership state.
    pub(super) fn new(
        id: OrganizationMembershipId,
        organization_id: OrganizationId,
        user_id: UserId,
    ) -> Self {
        Self {
            id,
            status: OrganizationMembershipStatus::Active,
            organization_id,
            user_id,
        }
    }
}

fn organization_user_values(
    state: &OrganizationMembershipState,
) -> Result<Option<UniqueValues>, OrganizationMembershipStateError> {
    if state.status.is_removed() {
        return Ok(None);
    }

    let organization_id = state.organization_id.value().to_string();
    let user_id = state.user_id.value().to_string();
    let organization_part = UniqueValuePart::try_from(organization_id.as_str())?;
    let user_part = UniqueValuePart::try_from(user_id.as_str())?;
    let value = UniqueValue::new(vec![organization_part, user_part])?;
    let values = UniqueValues::new(vec![value])?;

    Ok(Some(values))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, UniqueConstraints, UniqueKey, UniqueValues};

    use crate::{OrganizationId, UserId};

    use super::{
        OrganizationMembershipId, OrganizationMembershipState, OrganizationMembershipStatus,
    };

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = OrganizationMembershipId::new();
        let state = OrganizationMembershipState::new(id, OrganizationId::new(), UserId::new());

        assert_eq!(state.id(), id);
    }

    #[test]
    fn active_state_returns_unique_entries_for_organization_and_user() {
        let state = OrganizationMembershipState::new(
            OrganizationMembershipId::new(),
            OrganizationId::new(),
            UserId::new(),
        );

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(UniqueKey::new("organization_user"))
                .map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn removed_state_has_no_unique_entry() {
        let mut state = OrganizationMembershipState::new(
            OrganizationMembershipId::new(),
            OrganizationId::new(),
            UserId::new(),
        );
        state.status = OrganizationMembershipStatus::Removed;

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(UniqueKey::new("organization_user"))
                .map(UniqueValues::len),
            None
        );
    }
}
