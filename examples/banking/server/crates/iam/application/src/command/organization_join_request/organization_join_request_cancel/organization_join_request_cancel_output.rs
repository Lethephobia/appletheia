use banking_iam_domain::{
    OrganizationJoinRequestCancelRejectionReason, OrganizationJoinRequestCancelResult,
};
use serde::{Deserialize, Serialize};

/// The output returned after canceling an organization join request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationJoinRequestCancelOutput {
    Canceled,
    Rejected {
        reason: OrganizationJoinRequestCancelRejectionReason,
    },
}

impl From<OrganizationJoinRequestCancelResult> for OrganizationJoinRequestCancelOutput {
    fn from(value: OrganizationJoinRequestCancelResult) -> Self {
        match value {
            OrganizationJoinRequestCancelResult::Canceled => Self::Canceled,
            OrganizationJoinRequestCancelResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
