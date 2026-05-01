use banking_iam_domain::{
    OrganizationDisplayNameChangeRejectionReason, OrganizationDisplayNameChangeResult,
};
use serde::{Deserialize, Serialize};

/// Returned after an organization operation is handled.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationDisplayNameChangeOutput {
    Changed,
    Rejected {
        reason: OrganizationDisplayNameChangeRejectionReason,
    },
}

impl From<OrganizationDisplayNameChangeResult> for OrganizationDisplayNameChangeOutput {
    fn from(value: OrganizationDisplayNameChangeResult) -> Self {
        match value {
            OrganizationDisplayNameChangeResult::Changed => Self::Changed,
            OrganizationDisplayNameChangeResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
