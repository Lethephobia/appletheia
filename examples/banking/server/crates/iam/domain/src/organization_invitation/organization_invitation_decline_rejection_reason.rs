use serde::{Deserialize, Serialize};

/// Describes why an organization invitation decline request was rejected as a domain outcome.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationInvitationDeclineRejectionReason {
    Expired,
    NotPending,
}
