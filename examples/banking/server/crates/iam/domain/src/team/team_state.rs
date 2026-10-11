use appletheia::{aggregate_state, reference_indexes, unique_constraints};
use uuid::Uuid;

use super::{TeamDescription, TeamName, TeamStateError, TeamStatus};
use crate::OrganizationId;

#[aggregate_state(error = TeamStateError)]
#[unique_constraints]
#[reference_indexes(entry(key = "organization", value = organization_ref_value))]
pub struct TeamState {
    pub(super) organization_id: OrganizationId,
    pub(super) name: TeamName,
    pub(super) description: Option<TeamDescription>,
    pub(super) status: TeamStatus,
}

fn organization_ref_value(
    state: &TeamState,
    _aggregate_id: Uuid,
) -> Result<Option<OrganizationId>, TeamStateError> {
    Ok(Some(state.organization_id))
}
