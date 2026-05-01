use banking_iam_domain::{
    OrganizationJoinRequestRejectRejectionReason, OrganizationJoinRequestRejectResult,
};
use serde::{Deserialize, Serialize};

/// The output returned after rejecting an organization join request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationJoinRequestRejectOutput {
    Rejected,
    RejectionRejected {
        reason: OrganizationJoinRequestRejectRejectionReason,
    },
}

impl From<OrganizationJoinRequestRejectResult> for OrganizationJoinRequestRejectOutput {
    fn from(value: OrganizationJoinRequestRejectResult) -> Self {
        match value {
            OrganizationJoinRequestRejectResult::Rejected => Self::Rejected,
            OrganizationJoinRequestRejectResult::RejectionRejected { reason } => {
                Self::RejectionRejected { reason }
            }
        }
    }
}
