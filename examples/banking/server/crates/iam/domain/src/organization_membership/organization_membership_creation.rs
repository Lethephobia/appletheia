use crate::{OrganizationId, UserId};

use super::OrganizationRoles;

/// Describes the creation of an organization membership.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrganizationMembershipCreation {
    pub organization_id: OrganizationId,
    pub user_id: UserId,
    pub roles: OrganizationRoles,
}

impl OrganizationMembershipCreation {
    pub(super) fn into_parts(self) -> (OrganizationId, UserId, OrganizationRoles) {
        (self.organization_id, self.user_id, self.roles)
    }
}
