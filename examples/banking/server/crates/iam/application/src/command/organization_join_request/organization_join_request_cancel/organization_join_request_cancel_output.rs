use banking_iam_domain::OrganizationJoinRequestCancelRejectionReason;
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
