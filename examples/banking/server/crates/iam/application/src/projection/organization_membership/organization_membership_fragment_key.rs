use banking_iam_domain::{OrganizationId, UserId};
use serde::{Deserialize, Serialize};

/// Identifies one stored organization membership fragment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrganizationMembershipFragmentKey {
    pub user_id: UserId,
    pub organization_id: OrganizationId,
}
