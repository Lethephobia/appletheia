use banking_iam_domain::user::{
    UserIdentityEmailChangeRejectionReason, UserIdentityLinkRejectionReason,
};
use serde::{Deserialize, Serialize};

/// Describes why an OIDC completion flow was rejected as a business outcome.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OidcCompleteRejectionReason {
    IdentityLink {
        reason: UserIdentityLinkRejectionReason,
    },
    IdentityEmailChange {
        reason: UserIdentityEmailChangeRejectionReason,
    },
}
