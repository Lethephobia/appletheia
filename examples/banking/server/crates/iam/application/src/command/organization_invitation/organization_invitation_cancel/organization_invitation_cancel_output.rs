use banking_iam_domain::OrganizationInvitationCancelRejectionReason;
use serde::{Deserialize, Serialize};

/// The output returned after canceling an organization invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationInvitationCancelOutput {
    Canceled,
    Rejected {
        reason: OrganizationInvitationCancelRejectionReason,
    },
}
