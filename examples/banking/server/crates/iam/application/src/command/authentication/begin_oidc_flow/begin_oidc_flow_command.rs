use appletheia::application::authentication::PkceCodeChallenge;
use appletheia::application::authentication::oidc::{
    OidcDisplay, OidcExtraAuthorizeParams, OidcPrompt, OidcScopes,
};
use appletheia::command;
use serde::{Deserialize, Serialize};

use crate::oidc::{OidcCompletionPurpose, OidcCompletionRedirectUri, OidcReturnTo};

/// Starts an OIDC authorization flow for a user.
#[command(name = "begin_oidc_flow")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeginOidcFlowCommand {
    pub completion_purpose: OidcCompletionPurpose,
    pub completion_redirect_uri: OidcCompletionRedirectUri,
    pub return_to: Option<OidcReturnTo>,
    pub code_challenge: Option<PkceCodeChallenge>,
    pub scopes: OidcScopes,
    pub display: Option<OidcDisplay>,
    pub prompt: Option<OidcPrompt>,
    pub extra_authorize_params: OidcExtraAuthorizeParams,
}
