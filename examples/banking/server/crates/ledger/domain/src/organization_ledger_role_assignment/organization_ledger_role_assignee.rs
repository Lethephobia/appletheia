use super::{OrganizationLedgerRole, OrganizationLedgerRoleAssigneeError};
use appletheia::domain::UniqueValue;
use banking_iam_domain::OrganizationId;
use banking_iam_domain::{OrganizationMembershipId, TeamId};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationLedgerRoleAssignee {
    OrganizationMembership(OrganizationMembershipId),
    Team(TeamId),
}

impl OrganizationLedgerRoleAssignee {
    pub fn assignment_unique_value(
        self,
        organization_id: OrganizationId,
        role: OrganizationLedgerRole,
    ) -> Result<UniqueValue, OrganizationLedgerRoleAssigneeError> {
        let (kind, id) = match self {
            Self::OrganizationMembership(id) => ("organization_membership", id.to_string()),
            Self::Team(id) => ("team", id.to_string()),
        };
        Ok(UniqueValue::from_strings([
            organization_id.to_string().as_str(),
            kind,
            id.as_str(),
            role.as_str(),
        ])?)
    }
}
