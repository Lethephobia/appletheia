use serde::{Deserialize, Serialize};

use crate::oidc::{OidcCompletionPurpose, OidcCompletionRedirectUri, OidcReturnTo};

/// OIDC completion metadata retained for replay, excluding tokens and exchange codes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcCompleteReplayOutput {
    pub completion_purpose: OidcCompletionPurpose,
    pub completion_redirect_uri: OidcCompletionRedirectUri,
    pub return_to: Option<OidcReturnTo>,
}
