use banking_iam_domain::{OrganizationHandleChangeRejectionReason, OrganizationHandleChangeResult};
use serde::{Deserialize, Serialize};

/// Returned after an organization operation is handled.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationHandleChangeOutput {
    Changed,
    Rejected {
        reason: OrganizationHandleChangeRejectionReason,
    },
}

impl From<OrganizationHandleChangeResult> for OrganizationHandleChangeOutput {
    fn from(value: OrganizationHandleChangeResult) -> Self {
        match value {
            OrganizationHandleChangeResult::Changed => Self::Changed,
            OrganizationHandleChangeResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
