use super::{
    AuthTokenExchangeCode, AuthTokenExchangeCodeHasher, AuthTokenExchangeCodeIssueRequest,
    AuthTokenExchangeCodeIssueResult, AuthTokenExchangeCodeIssuer,
    AuthTokenExchangeCodeIssuerConfig, AuthTokenExchangeCodeIssuerError,
    AuthTokenExchangeCodeRecord, AuthTokenExchangeCodeStore, AuthTokenExchangeGrantCipher,
    PkceMode,
};

/// Issues encrypted one-time auth token exchange codes.
#[derive(Clone, Debug)]
pub struct DefaultAuthTokenExchangeCodeIssuer<S, C, H>
where
    S: AuthTokenExchangeCodeStore,
    C: AuthTokenExchangeGrantCipher,
    H: AuthTokenExchangeCodeHasher,
{
    config: AuthTokenExchangeCodeIssuerConfig,
    store: S,
    grant_cipher: C,
    code_hasher: H,
}

impl<S, C, H> DefaultAuthTokenExchangeCodeIssuer<S, C, H>
where
    S: AuthTokenExchangeCodeStore,
    C: AuthTokenExchangeGrantCipher,
    H: AuthTokenExchangeCodeHasher,
{
    /// Creates a new exchange code issuer.
    pub fn new(
        config: AuthTokenExchangeCodeIssuerConfig,
        store: S,
        grant_cipher: C,
        code_hasher: H,
    ) -> Self {
        Self {
            config,
            store,
            grant_cipher,
            code_hasher,
        }
    }
}

impl<S, C, H> AuthTokenExchangeCodeIssuer for DefaultAuthTokenExchangeCodeIssuer<S, C, H>
where
    S: AuthTokenExchangeCodeStore,
    C: AuthTokenExchangeGrantCipher,
    H: AuthTokenExchangeCodeHasher,
{
    type Uow = S::Uow;

    async fn issue(
        &self,
        uow: &mut Self::Uow,
        request: AuthTokenExchangeCodeIssueRequest,
    ) -> Result<AuthTokenExchangeCodeIssueResult, AuthTokenExchangeCodeIssuerError> {
        let code = AuthTokenExchangeCode::new();
        let code_hash = self.code_hasher.hash_code(&code)?;
        let encrypted_grant = self.grant_cipher.encrypt(request.grant()).await?;
        let protection = match (self.config.pkce_mode(), request.code_challenge().cloned()) {
            (
                PkceMode::Enabled {
                    code_challenge_method,
                },
                Some(challenge),
            ) => Some((code_challenge_method, challenge)),
            (PkceMode::Enabled { .. }, None) => {
                return Err(AuthTokenExchangeCodeIssuerError::MissingCodeChallenge);
            }
            (PkceMode::Disabled, Some(_)) => {
                return Err(AuthTokenExchangeCodeIssuerError::UnexpectedCodeChallenge);
            }
            (PkceMode::Disabled, None) => None,
        };
        let record = AuthTokenExchangeCodeRecord::new(
            code_hash,
            protection.as_ref().map(|(method, _)| *method),
            protection.map(|(_, challenge)| challenge),
            encrypted_grant,
            self.config.expires_in(),
        );

        self.store.save(uow, &record).await?;

        Ok(AuthTokenExchangeCodeIssueResult::new(
            code,
            record.expires_at(),
        ))
    }
}
