use banking_iam_domain::{OrganizationRemoveRejectionReason, OrganizationRemoveResult};
use serde::{Deserialize, Serialize};

/// Returned after an organization operation is handled.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationRemoveOutput {
    Removed,
    Rejected {
        reason: OrganizationRemoveRejectionReason,
    },
}

impl From<OrganizationRemoveResult> for OrganizationRemoveOutput {
    fn from(value: OrganizationRemoveResult) -> Self {
        match value {
            OrganizationRemoveResult::Removed => Self::Removed,
            OrganizationRemoveResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
