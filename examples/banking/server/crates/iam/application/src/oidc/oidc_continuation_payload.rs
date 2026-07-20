use appletheia::application::authentication::PkceCodeChallenge;
use serde::{Deserialize, Serialize};

use super::{OidcCompletionPurpose, OidcCompletionRedirectUri, OidcReturnTo};

/// Represents application-defined continuation payload for OIDC callbacks.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcContinuationPayload {
    pub completion_purpose: OidcCompletionPurpose,
    pub completion_redirect_uri: OidcCompletionRedirectUri,
    pub return_to: Option<OidcReturnTo>,
    pub code_challenge: Option<PkceCodeChallenge>,
}
