use serde::{Deserialize, Serialize};

use crate::oidc::{OidcCompletionPurpose, OidcCompletionRedirectUri};

/// Represents the replay-safe result returned after completing an OIDC flow.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcCompleteReplayOutput {
    pub completion_purpose: OidcCompletionPurpose,
    pub completion_redirect_uri: OidcCompletionRedirectUri,
}
