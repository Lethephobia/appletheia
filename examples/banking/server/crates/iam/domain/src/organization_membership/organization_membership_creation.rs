use crate::{OrganizationId, UserId};

use super::OrganizationMembershipRoles;

/// Describes an organization membership creation request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrganizationMembershipCreation {
    pub organization_id: OrganizationId,
    pub user_id: UserId,
    pub roles: OrganizationMembershipRoles,
}

impl OrganizationMembershipCreation {
    pub(super) fn into_parts(self) -> (OrganizationId, UserId, OrganizationMembershipRoles) {
        (self.organization_id, self.user_id, self.roles)
    }
}
