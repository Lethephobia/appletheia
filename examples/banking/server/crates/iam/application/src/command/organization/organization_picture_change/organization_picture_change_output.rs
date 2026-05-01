use banking_iam_domain::{
    OrganizationPictureChangeRejectionReason, OrganizationPictureChangeResult,
};
use serde::{Deserialize, Serialize};

/// Returned after an organization operation is handled.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationPictureChangeOutput {
    Changed,
    Rejected {
        reason: OrganizationPictureChangeRejectionReason,
    },
}

impl From<OrganizationPictureChangeResult> for OrganizationPictureChangeOutput {
    fn from(value: OrganizationPictureChangeResult) -> Self {
        match value {
            OrganizationPictureChangeResult::Changed => Self::Changed,
            OrganizationPictureChangeResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
