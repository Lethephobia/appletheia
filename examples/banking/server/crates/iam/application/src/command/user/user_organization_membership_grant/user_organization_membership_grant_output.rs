use banking_iam_domain::OrganizationMembershipGrantRejectionReason;
use serde::{Deserialize, Serialize};

/// Returned after granting an organization membership to a user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum UserOrganizationMembershipGrantOutput {
    Granted,
    Rejected {
        reason: OrganizationMembershipGrantRejectionReason,
    },
}
