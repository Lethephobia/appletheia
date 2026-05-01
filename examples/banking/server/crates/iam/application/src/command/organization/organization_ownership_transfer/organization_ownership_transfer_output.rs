use banking_iam_domain::{
    OrganizationOwnershipTransferRejectionReason, OrganizationOwnershipTransferResult,
};
use serde::{Deserialize, Serialize};

/// Returned after an organization operation is handled.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationOwnershipTransferOutput {
    Transferred,
    Rejected {
        reason: OrganizationOwnershipTransferRejectionReason,
    },
}

impl From<OrganizationOwnershipTransferResult> for OrganizationOwnershipTransferOutput {
    fn from(value: OrganizationOwnershipTransferResult) -> Self {
        match value {
            OrganizationOwnershipTransferResult::Transferred => Self::Transferred,
            OrganizationOwnershipTransferResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
