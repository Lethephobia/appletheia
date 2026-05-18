use appletheia::aggregate_state;
use appletheia::domain::{AggregateId, UniqueValue};
use appletheia::reference_indexes;
use appletheia::unique_constraints;

use crate::{OrganizationId, UserId};

use super::{
    OrganizationMembershipId, OrganizationMembershipRoles, OrganizationMembershipStateError,
    OrganizationMembershipStatus,
};

/// Stores the materialized state of an `OrganizationMembership` aggregate.
#[aggregate_state(error = OrganizationMembershipStateError)]
#[unique_constraints(entry(key = "organization_user", value = organization_user_unique_value))]
#[reference_indexes(
    entry(key = "organization", value = organization_ref_value),
    entry(key = "user", value = user_ref_value)
)]
pub struct OrganizationMembershipState {
    pub(super) id: OrganizationMembershipId,
    pub(super) organization_id: OrganizationId,
    pub(super) user_id: UserId,
    pub(super) roles: OrganizationMembershipRoles,
    pub(super) status: OrganizationMembershipStatus,
}

fn organization_user_unique_value(
    state: &OrganizationMembershipState,
) -> Result<Option<UniqueValue>, OrganizationMembershipStateError> {
    if state.status.is_removed() {
        return Ok(None);
    }

    let organization_id = state.organization_id.value().to_string();
    let user_id = state.user_id.value().to_string();
    let value = UniqueValue::from_strings([organization_id.as_str(), user_id.as_str()])?;

    Ok(Some(value))
}

fn organization_ref_value(
    state: &OrganizationMembershipState,
) -> Result<Option<OrganizationId>, OrganizationMembershipStateError> {
    Ok(Some(state.organization_id))
}

fn user_ref_value(
    state: &OrganizationMembershipState,
) -> Result<Option<UserId>, OrganizationMembershipStateError> {
    Ok(Some(state.user_id))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{
        AggregateState, ReferenceIndexes, ReferenceValues, UniqueConstraints, UniqueValues,
    };

    use crate::{OrganizationId, OrganizationRole, UserId};

    use super::{
        OrganizationMembershipId, OrganizationMembershipRoles, OrganizationMembershipState,
        OrganizationMembershipStatus,
    };

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = OrganizationMembershipId::new();
        let roles = OrganizationMembershipRoles::new([OrganizationRole::FinanceManager]);
        let state = OrganizationMembershipState {
            id,
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: roles.clone(),
            status: OrganizationMembershipStatus::Active,
        };

        assert_eq!(state.id(), id);
        assert_eq!(state.roles, roles);
    }

    #[test]
    fn active_state_returns_unique_entries_for_organization_and_user() {
        let state = OrganizationMembershipState {
            id: OrganizationMembershipId::new(),
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationMembershipRoles::default(),
            status: OrganizationMembershipStatus::Active,
        };

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(OrganizationMembershipState::ORGANIZATION_USER_KEY)
                .map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn removed_state_has_no_unique_entry() {
        let mut state = OrganizationMembershipState {
            id: OrganizationMembershipId::new(),
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationMembershipRoles::default(),
            status: OrganizationMembershipStatus::Active,
        };
        state.status = OrganizationMembershipStatus::Removed;

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(OrganizationMembershipState::ORGANIZATION_USER_KEY)
                .map(UniqueValues::len),
            None
        );
    }

    #[test]
    fn returns_reference_entries_for_organization_and_user() {
        let organization_id = OrganizationId::new();
        let user_id = UserId::new();
        let state = OrganizationMembershipState {
            id: OrganizationMembershipId::new(),
            organization_id,
            user_id,
            roles: OrganizationMembershipRoles::default(),
            status: OrganizationMembershipStatus::Active,
        };

        let entries = state
            .reference_entries()
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(OrganizationMembershipState::ORGANIZATION_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
        assert_eq!(
            entries
                .get(OrganizationMembershipState::USER_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
    }
}
