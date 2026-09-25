use appletheia::aggregate_state;
use appletheia::domain::{AggregateId, UniqueValue};
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use uuid::Uuid;

use crate::{OrganizationId, UserId};

use super::{OrganizationMembershipStateError, OrganizationMembershipStatus, OrganizationRoles};

/// Stores the materialized state of an `OrganizationMembership` aggregate.
#[aggregate_state(error = OrganizationMembershipStateError)]
#[unique_constraints(entry(key = "organization_user", value = organization_user_unique_value))]
#[reference_indexes(
    entry(key = "organization", value = organization_ref_value),
    entry(key = "user", value = user_ref_value)
)]
pub struct OrganizationMembershipState {
    pub(super) organization_id: OrganizationId,
    pub(super) user_id: UserId,
    pub(super) roles: OrganizationRoles,
    pub(super) status: OrganizationMembershipStatus,
}

/// Reserves the organization/user pair for the currently effective membership only.
///
/// A removed membership releases the pair so the same user can rejoin the
/// organization through a new membership aggregate.
fn organization_user_unique_value(
    state: &OrganizationMembershipState,
    _aggregate_id: Uuid,
) -> Result<Option<UniqueValue>, OrganizationMembershipStateError> {
    if !state.status.is_active() {
        return Ok(None);
    }

    let organization_id = state.organization_id.value().to_string();
    let user_id = state.user_id.value().to_string();
    let value = UniqueValue::from_strings([organization_id.as_str(), user_id.as_str()])?;

    Ok(Some(value))
}

fn organization_ref_value(
    state: &OrganizationMembershipState,
    _aggregate_id: Uuid,
) -> Result<Option<OrganizationId>, OrganizationMembershipStateError> {
    Ok(Some(state.organization_id))
}

fn user_ref_value(
    state: &OrganizationMembershipState,
    _aggregate_id: Uuid,
) -> Result<Option<UserId>, OrganizationMembershipStateError> {
    Ok(Some(state.user_id))
}
