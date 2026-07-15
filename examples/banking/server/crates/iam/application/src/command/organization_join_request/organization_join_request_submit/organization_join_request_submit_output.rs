use banking_iam_domain::{OrganizationJoinRequestId, OrganizationJoinRequestSubmitRejectionReason};
use serde::{Deserialize, Serialize};

/// The output returned after submitting an organization join request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationJoinRequestSubmitOutput {
    Submitted {
        organization_join_request_id: OrganizationJoinRequestId,
    },
    Rejected {
        organization_join_request_id: OrganizationJoinRequestId,
        reason: OrganizationJoinRequestSubmitRejectionReason,
    },
}
