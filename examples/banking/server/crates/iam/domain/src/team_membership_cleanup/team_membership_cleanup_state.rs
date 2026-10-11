use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use uuid::Uuid;

use crate::{OrganizationMembershipId, TeamMembershipId};

use super::{
    TeamMembershipCleanupCount, TeamMembershipCleanupStateError, TeamMembershipCleanupStatus,
};

#[aggregate_state(error = TeamMembershipCleanupStateError)]
#[unique_constraints()]
#[reference_indexes(entry(key = "organization_membership", value = organization_membership_ref_value))]
pub struct TeamMembershipCleanupState {
    pub(super) organization_membership_id: OrganizationMembershipId,
    pub(super) requested_count: TeamMembershipCleanupCount,
    pub(super) succeeded_count: TeamMembershipCleanupCount,
    pub(super) failed_count: TeamMembershipCleanupCount,
    pub(super) next_cursor: Option<TeamMembershipId>,
    pub(super) scan_completed: bool,
    pub(super) status: TeamMembershipCleanupStatus,
}

fn organization_membership_ref_value(
    state: &TeamMembershipCleanupState,
    _aggregate_id: Uuid,
) -> Result<Option<OrganizationMembershipId>, TeamMembershipCleanupStateError> {
    Ok(Some(state.organization_membership_id))
}
