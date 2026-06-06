use serde::{Deserialize, Serialize};

use crate::OrganizationId;

use super::OrganizationRoles;

/// Describes an organization membership grant for a `User`.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct OrganizationMembershipGrant {
    pub organization_id: OrganizationId,
    pub roles: OrganizationRoles,
}

impl OrganizationMembershipGrant {
    pub(crate) fn into_parts(self) -> (OrganizationId, OrganizationRoles) {
        (self.organization_id, self.roles)
    }
}
