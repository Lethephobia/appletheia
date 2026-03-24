use super::{
    OidcAuthorizationUrlBuilder, OidcBeginOptions, OidcBeginResult, OidcCallbackParams,
    OidcCompleteResult, OidcIdTokenVerifier, OidcIdTokenVerifyContext, OidcLoginAttempt,
    OidcLoginAttemptCreatedAt, OidcLoginAttemptExpiresAt, OidcLoginAttemptStore, OidcLoginFlow,
    OidcLoginFlowConfig, OidcLoginFlowError, OidcNonce, OidcProviderMetadataSource, OidcScopes,
    OidcState, OidcTokenClient, OidcTokenGrant, OidcTokenRequest, OidcTokenResponse, OidcTokens,
    PkceCodeVerifier, PkceMode,
};

pub struct DefaultOidcLoginFlow<LAS, PMS, TC, ITV>
where
    LAS: OidcLoginAttemptStore,
    PMS: OidcProviderMetadataSource,
    TC: OidcTokenClient,
    ITV: OidcIdTokenVerifier,
{
    login_flow_config: OidcLoginFlowConfig,
    login_attempt_store: LAS,
    provider_metadata_source: PMS,
    token_client: TC,
    id_token_verifier: ITV,
}

impl<LAS, PMS, TC, ITV> DefaultOidcLoginFlow<LAS, PMS, TC, ITV>
where
    LAS: OidcLoginAttemptStore,
    PMS: OidcProviderMetadataSource,
    TC: OidcTokenClient,
    ITV: OidcIdTokenVerifier,
{
    pub fn new(
        login_flow_config: OidcLoginFlowConfig,
        login_attempt_store: LAS,
        provider_metadata_source: PMS,
        token_client: TC,
        id_token_verifier: ITV,
    ) -> Self {
        Self {
            login_flow_config,
            login_attempt_store,
            provider_metadata_source,
            token_client,
            id_token_verifier,
        }
    }
}

impl<LAS, PMS, TC, ITV> OidcLoginFlow for DefaultOidcLoginFlow<LAS, PMS, TC, ITV>
where
    LAS: OidcLoginAttemptStore,
    PMS: OidcProviderMetadataSource,
    TC: OidcTokenClient,
    ITV: OidcIdTokenVerifier,
{
    type Uow = LAS::Uow;

    async fn begin(
        &self,
        uow: &mut Self::Uow,
        mut options: OidcBeginOptions,
    ) -> Result<OidcBeginResult, OidcLoginFlowError> {
        options.scopes = OidcScopes::new(options.scopes.values().to_vec());

        let now = OidcLoginAttemptCreatedAt::now();
        let issuer_url = &self.login_flow_config.provider_config.issuer_url;
        let provider_metadata = self
            .provider_metadata_source
            .read_provider_metadata(issuer_url)
            .await?;

        let state = OidcState::new();
        let nonce = OidcNonce::new();

        let (pkce_code_verifier, pkce_code_challenge) =
            match self.login_flow_config.provider_config.pkce_mode {
                PkceMode::Disabled => (None, None),
                PkceMode::Enabled {
                    code_challenge_method,
                } => {
                    let verifier = PkceCodeVerifier::new();
                    let challenge = verifier.to_code_challenge(code_challenge_method);
                    (Some(verifier), Some((challenge, code_challenge_method)))
                }
            };

        let authorization_url_builder = OidcAuthorizationUrlBuilder::new(
            provider_metadata.authorization_endpoint_url.clone(),
            self.login_flow_config.provider_config.client_id.clone(),
            self.login_flow_config.provider_config.redirect_uri.clone(),
            options.scopes.clone(),
            state.clone(),
            nonce.clone(),
        );

        let authorization_url_builder = match options.display {
            Some(display) => authorization_url_builder.with_display(display),
            None => authorization_url_builder,
        };

        let authorization_url_builder = match options.max_age {
            Some(max_age) => authorization_url_builder.with_max_age(max_age),
            None => authorization_url_builder,
        };

        let authorization_url_builder = match options.prompt {
            Some(prompt) => authorization_url_builder.with_prompt(prompt),
            None => authorization_url_builder,
        };

        let authorization_url_builder = match pkce_code_challenge {
            Some((challenge, method)) => authorization_url_builder.with_pkce(challenge, method),
            None => authorization_url_builder,
        };

        let authorization_url_builder = options.extra_authorize_params.into_iter().fold(
            authorization_url_builder,
            |authorization_url_builder, (key, value)| {
                authorization_url_builder.with_extra_authorize_param(key, value)
            },
        );

        let authorization_url = authorization_url_builder.build();

        let expires_at = OidcLoginAttemptExpiresAt::from_created_at(
            now,
            self.login_flow_config.login_attempt_expires_in,
        );

        let attempt = OidcLoginAttempt::new(state, nonce, pkce_code_verifier, now, expires_at);

        self.login_attempt_store.save(uow, &attempt).await?;

        Ok(OidcBeginResult {
            authorization_url,
            expires_at,
        })
    }

    async fn complete(
        &self,
        uow: &mut Self::Uow,
        callback_params: OidcCallbackParams,
    ) -> Result<OidcCompleteResult, OidcLoginFlowError> {
        let state = callback_params.state.clone();
        let attempt = self
            .login_attempt_store
            .consume_by_state(uow, &state)
            .await?;

        let issuer_url = &self.login_flow_config.provider_config.issuer_url;
        let provider_metadata = self
            .provider_metadata_source
            .read_provider_metadata(issuer_url)
            .await?;

        let token_request = OidcTokenRequest {
            token_endpoint_url: provider_metadata.token_endpoint_url.value().clone(),
            client_id: self.login_flow_config.provider_config.client_id.clone(),
            client_auth: self.login_flow_config.provider_config.client_auth.clone(),
            grant: OidcTokenGrant::AuthorizationCode {
                authorization_code: callback_params.authorization_code,
                redirect_uri: self.login_flow_config.provider_config.redirect_uri.clone(),
                pkce_code_verifier: attempt.pkce_code_verifier().cloned(),
            },
        };

        let token_response = self.token_client.request_token(token_request).await?;
        let OidcTokenResponse {
            id_token,
            access_token,
            refresh_token,
            expires_in,
        } = token_response;

        let id_token = id_token.ok_or(OidcLoginFlowError::MissingIdToken)?;

        let verify_context = OidcIdTokenVerifyContext {
            issuer_url: self.login_flow_config.provider_config.issuer_url.clone(),
            client_id: self.login_flow_config.provider_config.client_id.clone(),
            jwks_uri: provider_metadata.jwks_uri.clone(),
            access_token: access_token.clone(),
            expected_nonce: Some(attempt.nonce().clone()),
        };

        let id_token_claims = self
            .id_token_verifier
            .verify(&id_token, verify_context)
            .await?;

        Ok(OidcCompleteResult {
            tokens: OidcTokens::new(id_token, access_token, refresh_token, expires_in),
            id_token_claims,
        })
    }
}
