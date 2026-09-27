use serde::Serialize;

use appletheia::domain::EventOccurredAt;
use banking_iam_domain::OrganizationRoles;

use super::OrganizationMemberListMember;

/// One organization member list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationMemberListItem {
    pub member: OrganizationMemberListMember,
    pub roles: OrganizationRoles,
    pub is_owner: bool,
    pub joined_at: EventOccurredAt,
}
