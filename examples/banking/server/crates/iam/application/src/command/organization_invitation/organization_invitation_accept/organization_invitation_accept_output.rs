use banking_iam_domain::{
    OrganizationInvitationAcceptRejectionReason, OrganizationInvitationAcceptResult,
};
use serde::{Deserialize, Serialize};

/// The output returned after accepting an organization invitation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationInvitationAcceptOutput {
    Accepted,
    Rejected {
        reason: OrganizationInvitationAcceptRejectionReason,
    },
}

impl From<OrganizationInvitationAcceptResult> for OrganizationInvitationAcceptOutput {
    fn from(value: OrganizationInvitationAcceptResult) -> Self {
        match value {
            OrganizationInvitationAcceptResult::Accepted => Self::Accepted,
            OrganizationInvitationAcceptResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
