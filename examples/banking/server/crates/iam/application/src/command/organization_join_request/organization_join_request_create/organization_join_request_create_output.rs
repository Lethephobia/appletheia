use banking_iam_domain::{
    OrganizationJoinRequestId, OrganizationJoinRequestRequestRejectionReason,
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
        organization_join_request_id: OrganizationJoinRequestId,
        reason: OrganizationJoinRequestRequestRejectionReason,
    },
}
