use super::{OrganizationRole, OrganizationRoleAssigneeError};
use crate::OrganizationId;
use crate::OrganizationMembershipId;
use appletheia::domain::UniqueValue;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationRoleAssignee {
    OrganizationMembership(OrganizationMembershipId),
}

impl OrganizationRoleAssignee {
    pub fn assignment_unique_value(
        self,
        organization_id: OrganizationId,
        role: OrganizationRole,
    ) -> Result<UniqueValue, OrganizationRoleAssigneeError> {
        let (kind, id) = match self {
            Self::OrganizationMembership(id) => ("organization_membership", id.to_string()),
        };
        Ok(UniqueValue::from_strings([
            organization_id.to_string().as_str(),
            kind,
            id.as_str(),
            role.as_str(),
        ])?)
    }
}
