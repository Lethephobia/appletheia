use banking_iam_domain::OrganizationDisplayNameChangeRejectionReason;
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
