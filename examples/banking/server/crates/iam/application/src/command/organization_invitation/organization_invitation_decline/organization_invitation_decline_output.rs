use banking_iam_domain::{
    OrganizationInvitationDeclineRejectionReason, OrganizationInvitationDeclineResult,
};
use serde::{Deserialize, Serialize};

/// The output returned after declining an organization invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationInvitationDeclineOutput {
    Declined,
    Rejected {
        reason: OrganizationInvitationDeclineRejectionReason,
    },
}

impl From<OrganizationInvitationDeclineResult> for OrganizationInvitationDeclineOutput {
    fn from(value: OrganizationInvitationDeclineResult) -> Self {
        match value {
            OrganizationInvitationDeclineResult::Declined => Self::Declined,
            OrganizationInvitationDeclineResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
