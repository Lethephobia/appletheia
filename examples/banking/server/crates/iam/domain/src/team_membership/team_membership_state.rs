use appletheia::aggregate_state;
use appletheia::domain::{AggregateId, UniqueValue};
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use uuid::Uuid;

use crate::{OrganizationMembershipId, TeamId};

use super::{TeamMembershipStateError, TeamMembershipStatus};

#[aggregate_state(error = TeamMembershipStateError)]
#[unique_constraints(entry(key = "team_organization_membership", value = team_organization_membership_unique_value))]
#[reference_indexes(
    entry(key = "team", value = team_ref_value),
    entry(key = "organization_membership", value = organization_membership_ref_value)
)]
pub struct TeamMembershipState {
    pub(super) team_id: TeamId,
    pub(super) organization_membership_id: OrganizationMembershipId,
    pub(super) status: TeamMembershipStatus,
}

fn team_organization_membership_unique_value(
    state: &TeamMembershipState,
    _aggregate_id: Uuid,
) -> Result<Option<UniqueValue>, TeamMembershipStateError> {
    if !state.status.is_active() {
        return Ok(None);
    }

    let team_id = state.team_id.value().to_string();
    let organization_membership_id = state.organization_membership_id.value().to_string();
    let value = UniqueValue::from_strings([team_id.as_str(), organization_membership_id.as_str()])?;

    Ok(Some(value))
}

fn team_ref_value(
    state: &TeamMembershipState,
    _aggregate_id: Uuid,
) -> Result<Option<TeamId>, TeamMembershipStateError> {
    Ok(Some(state.team_id))
}

fn organization_membership_ref_value(
    state: &TeamMembershipState,
    _aggregate_id: Uuid,
) -> Result<Option<OrganizationMembershipId>, TeamMembershipStateError> {
    Ok(Some(state.organization_membership_id))
}
