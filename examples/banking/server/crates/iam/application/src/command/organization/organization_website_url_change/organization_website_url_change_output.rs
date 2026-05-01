use banking_iam_domain::{
    OrganizationWebsiteUrlChangeRejectionReason, OrganizationWebsiteUrlChangeResult,
};
use serde::{Deserialize, Serialize};

/// Returned after an organization operation is handled.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationWebsiteUrlChangeOutput {
    Changed,
    Rejected {
        reason: OrganizationWebsiteUrlChangeRejectionReason,
    },
}

impl From<OrganizationWebsiteUrlChangeResult> for OrganizationWebsiteUrlChangeOutput {
    fn from(value: OrganizationWebsiteUrlChangeResult) -> Self {
        match value {
            OrganizationWebsiteUrlChangeResult::Changed => Self::Changed,
            OrganizationWebsiteUrlChangeResult::Rejected { reason } => Self::Rejected { reason },
        }
    }
}
