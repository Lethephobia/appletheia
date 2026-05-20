use banking_iam_domain::OrganizationJoinRequestApproveRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after approving an organization join request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationJoinRequestApproveOutput {
    Approved,
    Rejected {
        reason: OrganizationJoinRequestApproveRejectionReason,
    },
}
