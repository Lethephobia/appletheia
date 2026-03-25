use appletheia::application::authentication::PkceCodeChallenge;
use banking_iam_domain::UserId;
use serde::{Deserialize, Serialize};

use super::{OidcCompletionPurpose, OidcCompletionRedirectUri};

/// Represents application-defined continuation payload for OIDC callbacks.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcContinuationPayload {
    pub completion_purpose: OidcCompletionPurpose,
    pub completion_redirect_uri: OidcCompletionRedirectUri,
    pub code_challenge: Option<PkceCodeChallenge>,
    pub principal_user_id: Option<UserId>,
}
