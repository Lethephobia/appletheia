use appletheia::event_payload;

use super::{TeamMembershipCleanupCount, TeamMembershipCleanupEventPayloadError};
use crate::{OrganizationMembershipId, TeamMembershipId};

#[event_payload(error = TeamMembershipCleanupEventPayloadError)]
pub enum TeamMembershipCleanupEventPayload {
    Started {
        organization_membership_id: OrganizationMembershipId,
    },
    Scanned {
        team_membership_ids: Vec<TeamMembershipId>,
        next_cursor: Option<TeamMembershipId>,
    },
    MembershipSucceeded {
        team_membership_id: TeamMembershipId,
    },
    MembershipFailed {
        team_membership_id: TeamMembershipId,
    },
    Completed {
        succeeded_count: TeamMembershipCleanupCount,
        failed_count: TeamMembershipCleanupCount,
    },
}
