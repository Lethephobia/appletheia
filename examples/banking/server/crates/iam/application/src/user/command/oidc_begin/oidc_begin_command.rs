use std::collections::BTreeMap;

use appletheia::application::authentication::PkceCodeChallenge;
use appletheia::application::authentication::oidc::{OidcDisplay, OidcPrompt, OidcScopes};
use appletheia::application::command::{Command, CommandName};
use serde::{Deserialize, Serialize};

use crate::user::{OidcCompletionMode, OidcCompletionRedirectUri};

/// Starts an OIDC authorization flow for a user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcBeginCommand {
    pub completion_mode: OidcCompletionMode,
    pub completion_redirect_uri: OidcCompletionRedirectUri,
    pub code_challenge: Option<PkceCodeChallenge>,
    pub scopes: OidcScopes,
    pub display: Option<OidcDisplay>,
    pub prompt: Option<OidcPrompt>,
    pub extra_authorize_params: BTreeMap<String, String>,
}

impl Command for OidcBeginCommand {
    const NAME: CommandName = CommandName::new("oidc_begin");
}
