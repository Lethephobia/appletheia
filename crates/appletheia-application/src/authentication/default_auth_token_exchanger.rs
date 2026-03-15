use super::{
    AuthTokenExchangeCodeHasher, AuthTokenExchangeCodeStore, AuthTokenExchangeGrantCipher,
    AuthTokenExchangeRequest, AuthTokenExchangeResult, AuthTokenExchanger, AuthTokenExchangerError,
    AuthTokenIssueRequest, AuthTokenIssuer,
};

/// Exchanges one-time codes for auth tokens and optional OIDC tokens.
#[derive(Clone, Debug)]
pub struct DefaultAuthTokenExchanger<S, C, I, H>
where
    S: AuthTokenExchangeCodeStore,
    C: AuthTokenExchangeGrantCipher,
    I: AuthTokenIssuer,
    H: AuthTokenExchangeCodeHasher,
{
    store: S,
    grant_cipher: C,
    auth_token_issuer: I,
    code_hasher: H,
}

impl<S, C, I, H> DefaultAuthTokenExchanger<S, C, I, H>
where
    S: AuthTokenExchangeCodeStore,
    C: AuthTokenExchangeGrantCipher,
    I: AuthTokenIssuer,
    H: AuthTokenExchangeCodeHasher,
{
    /// Creates a new auth token exchanger.
    pub fn new(store: S, grant_cipher: C, auth_token_issuer: I, code_hasher: H) -> Self {
        Self {
            store,
            grant_cipher,
            auth_token_issuer,
            code_hasher,
        }
    }
}

impl<S, C, I, H> AuthTokenExchanger for DefaultAuthTokenExchanger<S, C, I, H>
where
    S: AuthTokenExchangeCodeStore,
    C: AuthTokenExchangeGrantCipher,
    I: AuthTokenIssuer,
    H: AuthTokenExchangeCodeHasher,
{
    type Uow = S::Uow;

    async fn exchange(
        &self,
        uow: &mut Self::Uow,
        request: AuthTokenExchangeRequest,
    ) -> Result<AuthTokenExchangeResult, AuthTokenExchangerError> {
        let code_hash = self.code_hasher.hash_code(request.code())?;
        let record = self.store.consume_by_code_hash(uow, &code_hash).await?;

        match (record.code_challenge_method(), request.code_verifier()) {
            (Some(method), Some(code_verifier)) => {
                let expected = code_verifier.to_code_challenge(method);
                let Some(actual) = record.code_challenge() else {
                    return Err(AuthTokenExchangerError::InvalidCodeVerifier);
                };
                if &expected != actual {
                    return Err(AuthTokenExchangerError::InvalidCodeVerifier);
                }
            }
            (Some(_), None) => return Err(AuthTokenExchangerError::MissingCodeVerifier),
            (None, Some(_)) => return Err(AuthTokenExchangerError::UnexpectedCodeVerifier),
            (None, None) => {}
        }

        let grant = self.grant_cipher.decrypt(record.encrypted_grant()).await?;
        let auth_token_issue_result = self
            .auth_token_issuer
            .issue(AuthTokenIssueRequest::new(grant.subject().clone()))
            .await?;

        Ok(AuthTokenExchangeResult::new(
            auth_token_issue_result,
            grant.oidc_tokens().cloned(),
        ))
    }
}
