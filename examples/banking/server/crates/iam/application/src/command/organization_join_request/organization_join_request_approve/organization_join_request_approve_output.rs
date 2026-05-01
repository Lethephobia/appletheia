use banking_iam_domain::{
    OrganizationJoinRequestApproveRejectionReason, OrganizationJoinRequestApproveResult,
};
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

impl From<OrganizationJoinRequestApproveResult> for OrganizationJoinRequestApproveOutput {
    fn from(value: OrganizationJoinRequestApproveResult) -> Self {
        match value {
            OrganizationJoinRequestApproveResult::Approved => Self::Approved,
            OrganizationJoinRequestApproveResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
