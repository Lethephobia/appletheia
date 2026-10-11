use appletheia::event_payload;

use super::{TeamDescription, TeamEventPayloadError, TeamName};
use crate::OrganizationId;

#[event_payload(error = TeamEventPayloadError)]
pub enum TeamEventPayload {
    Created {
        organization_id: OrganizationId,
        name: TeamName,
    },
    NameChanged {
        name: TeamName,
    },
    DescriptionSet {
        description: Option<TeamDescription>,
    },
    Removed,
}
