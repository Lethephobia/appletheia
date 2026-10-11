use super::{TeamRole, TeamRoleAssigneeError};
use crate::TeamId;
use crate::TeamMembershipId;
use appletheia::domain::UniqueValue;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum TeamRoleAssignee {
    TeamMembership(TeamMembershipId),
}

impl TeamRoleAssignee {
    pub fn assignment_unique_value(
        self,
        team_id: TeamId,
        role: TeamRole,
    ) -> Result<UniqueValue, TeamRoleAssigneeError> {
        let (kind, id) = match self {
            Self::TeamMembership(id) => ("team_membership", id.to_string()),
        };
        Ok(UniqueValue::from_strings([
            team_id.to_string().as_str(),
            kind,
            id.as_str(),
            role.as_str(),
        ])?)
    }
}
