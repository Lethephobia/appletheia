use appletheia::application::authentication::PkceCodeChallenge;
use serde::{Deserialize, Serialize};

use super::{OidcCompletionMode, OidcCompletionRedirectUri};

/// Represents application-defined continuation payload for OIDC callbacks.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcContinuationPayload {
    pub completion_mode: OidcCompletionMode,
    pub completion_redirect_uri: OidcCompletionRedirectUri,
    pub code_challenge: Option<PkceCodeChallenge>,
}
