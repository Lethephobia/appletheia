use banking_iam_domain::{
    OrganizationJoinRequestId, OrganizationJoinRequestRequestRejectionReason,
    OrganizationJoinRequestRequestResult,
};
use serde::{Deserialize, Serialize};

/// The output returned after creating an organization join request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationJoinRequestCreateOutput {
    Requested {
        organization_join_request_id: OrganizationJoinRequestId,
    },
    Rejected {
        reason: OrganizationJoinRequestRequestRejectionReason,
    },
}

impl From<OrganizationJoinRequestRequestResult> for OrganizationJoinRequestCreateOutput {
    fn from(value: OrganizationJoinRequestRequestResult) -> Self {
        match value {
            OrganizationJoinRequestRequestResult::Requested {
                organization_join_request_id,
            } => Self::Requested {
                organization_join_request_id,
            },
            OrganizationJoinRequestRequestResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
