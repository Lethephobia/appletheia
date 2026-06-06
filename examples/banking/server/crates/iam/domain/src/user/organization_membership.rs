mod organization_membership_grant;
mod organization_membership_grant_rejection_reason;
mod organization_membership_grant_result;
mod organization_membership_remove_rejection_reason;
mod organization_membership_remove_result;
mod organization_membership_roles_change_rejection_reason;
mod organization_membership_roles_change_result;
mod organization_role;
mod organization_roles;

pub use organization_membership_grant::OrganizationMembershipGrant;
pub use organization_membership_grant_rejection_reason::OrganizationMembershipGrantRejectionReason;
pub use organization_membership_grant_result::OrganizationMembershipGrantResult;
pub use organization_membership_remove_rejection_reason::OrganizationMembershipRemoveRejectionReason;
pub use organization_membership_remove_result::OrganizationMembershipRemoveResult;
pub use organization_membership_roles_change_rejection_reason::OrganizationMembershipRolesChangeRejectionReason;
pub use organization_membership_roles_change_result::OrganizationMembershipRolesChangeResult;
pub use organization_role::OrganizationRole;
pub use organization_roles::OrganizationRoles;

use serde::{Deserialize, Serialize};

use crate::OrganizationId;

/// Represents one organization membership owned by a user.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct OrganizationMembership {
    organization_id: OrganizationId,
    roles: OrganizationRoles,
}

impl OrganizationMembership {
    pub fn new(organization_id: OrganizationId, roles: OrganizationRoles) -> Self {
        Self {
            organization_id,
            roles,
        }
    }

    pub fn organization_id(&self) -> OrganizationId {
        self.organization_id
    }

    pub fn roles(&self) -> &OrganizationRoles {
        &self.roles
    }

    pub(super) fn change_roles(&mut self, roles: OrganizationRoles) {
        self.roles = roles;
    }
}
