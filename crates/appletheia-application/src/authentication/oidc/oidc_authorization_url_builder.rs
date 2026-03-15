use std::collections::BTreeMap;

use super::{
    OidcAuthorizationEndpointUrl, OidcAuthorizationUrl, OidcClientId, OidcDisplay, OidcMaxAge,
    OidcNonce, OidcPrompt, OidcRedirectUri, OidcResponseType, OidcScopes, OidcState,
    PkceCodeChallenge, PkceCodeChallengeMethod,
};

#[derive(Clone, Debug)]
pub struct OidcAuthorizationUrlBuilder {
    authorization_endpoint_url: OidcAuthorizationEndpointUrl,
    client_id: OidcClientId,
    redirect_uri: OidcRedirectUri,
    scopes: OidcScopes,
    state: OidcState,
    nonce: OidcNonce,
    display: Option<OidcDisplay>,
    max_age: Option<OidcMaxAge>,
    prompt: Option<OidcPrompt>,
    pkce_code_challenge: Option<(PkceCodeChallenge, PkceCodeChallengeMethod)>,
    extra_authorize_params: BTreeMap<String, String>,
}

impl OidcAuthorizationUrlBuilder {
    pub fn new(
        authorization_endpoint_url: OidcAuthorizationEndpointUrl,
        client_id: OidcClientId,
        redirect_uri: OidcRedirectUri,
        scopes: OidcScopes,
        state: OidcState,
        nonce: OidcNonce,
    ) -> Self {
        Self {
            authorization_endpoint_url,
            client_id,
            redirect_uri,
            scopes,
            state,
            nonce,
            display: None,
            max_age: None,
            prompt: None,
            pkce_code_challenge: None,
            extra_authorize_params: BTreeMap::new(),
        }
    }

    pub fn with_display(mut self, display: OidcDisplay) -> Self {
        self.display = Some(display);
        self
    }

    pub fn with_max_age(mut self, max_age: OidcMaxAge) -> Self {
        self.max_age = Some(max_age);
        self
    }

    pub fn with_prompt(mut self, prompt: OidcPrompt) -> Self {
        self.prompt = Some(prompt);
        self
    }

    pub fn with_pkce(
        mut self,
        code_challenge: PkceCodeChallenge,
        method: PkceCodeChallengeMethod,
    ) -> Self {
        self.pkce_code_challenge = Some((code_challenge, method));
        self
    }

    pub fn with_extra_authorize_param(mut self, key: String, value: String) -> Self {
        self.extra_authorize_params.insert(key, value);
        self
    }

    pub fn build(self) -> OidcAuthorizationUrl {
        let mut url = self.authorization_endpoint_url.value().clone();

        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("response_type", OidcResponseType::Code.as_str());
            pairs.append_pair("client_id", self.client_id.value());
            pairs.append_pair("redirect_uri", self.redirect_uri.value().as_str());
            pairs.append_pair("scope", &self.scopes.to_scope_string());
            pairs.append_pair("state", self.state.value());
            pairs.append_pair("nonce", self.nonce.value());

            if let Some(display) = self.display {
                pairs.append_pair("display", display.as_str());
            }

            if let Some(max_age) = self.max_age {
                let max_age = max_age.value().num_seconds().to_string();
                pairs.append_pair("max_age", &max_age);
            }

            if let Some(prompt) = self.prompt {
                pairs.append_pair("prompt", prompt.as_str());
            }

            if let Some((challenge, method)) = self.pkce_code_challenge {
                pairs.append_pair("code_challenge", challenge.value());
                pairs.append_pair("code_challenge_method", method.value());
            }

            for (key, value) in self.extra_authorize_params {
                pairs.append_pair(&key, &value);
            }
        }

        OidcAuthorizationUrl::new(url)
    }
}
