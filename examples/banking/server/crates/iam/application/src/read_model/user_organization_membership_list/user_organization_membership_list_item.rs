use serde::Serialize;

use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{OrganizationMembershipId, OrganizationRoles};

use super::UserOrganizationMembershipListOrganization;

/// One user organization membership list row.
///
/// `organization_membership_id` is the aggregate identifier that membership
/// commands address, so a caller can act on the row it just read.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserOrganizationMembershipListItem {
    pub organization_membership_id: OrganizationMembershipId,
    pub organization: UserOrganizationMembershipListOrganization,
    pub roles: OrganizationRoles,
    pub created_at: EventOccurredAt,
}
