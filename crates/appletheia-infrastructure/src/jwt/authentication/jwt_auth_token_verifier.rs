use appletheia_application::{AuthToken, AuthTokenClaims, AuthTokenVerifier, AuthTokenVerifyError};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};

use super::jwt_auth_token_claims::JwtAuthTokenClaims;
use super::jwt_auth_token_verifier_config::JwtAuthTokenVerifierConfig;
use super::jwt_auth_token_verifier_error::JwtAuthTokenVerifierError;

#[derive(Clone, Debug)]
pub struct JwtAuthTokenVerifier {
    config: JwtAuthTokenVerifierConfig,
}

impl JwtAuthTokenVerifier {
    pub fn new(config: JwtAuthTokenVerifierConfig) -> Self {
        Self { config }
    }

    fn validation(&self) -> Validation {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.algorithms = vec![Algorithm::RS256, Algorithm::EdDSA];
        validation.leeway = self.config.leeway_seconds().value();
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);

        let issuers: Vec<String> = self
            .config
            .allowed_issuer_urls()
            .iter()
            .map(|url| url.value().to_string())
            .collect();
        validation.set_issuer(&issuers);

        let audiences: Vec<String> = self
            .config
            .allowed_audiences()
            .values()
            .iter()
            .map(|aud| aud.value().to_owned())
            .collect();
        validation.set_audience(&audiences);

        validation
    }
}

impl AuthTokenVerifier for JwtAuthTokenVerifier {
    async fn verify(&self, token: &AuthToken) -> Result<AuthTokenClaims, AuthTokenVerifyError> {
        let token_value = token.value();

        let header = decode_header(token_value).map_err(|e| {
            AuthTokenVerifyError::Backend(Box::new(JwtAuthTokenVerifierError::DecodeHeader(e)))
        })?;

        let key_id = header.kid.ok_or_else(|| {
            AuthTokenVerifyError::Backend(Box::new(JwtAuthTokenVerifierError::MissingKeyId))
        })?;

        let jwk = self.config.jwks().find(&key_id).ok_or_else(|| {
            AuthTokenVerifyError::Backend(Box::new(JwtAuthTokenVerifierError::UnknownKeyId))
        })?;

        let decoding_key = DecodingKey::from_jwk(jwk).map_err(|e| {
            AuthTokenVerifyError::Backend(Box::new(JwtAuthTokenVerifierError::InvalidKey(e)))
        })?;

        let validation = self.validation();
        let token_data = decode::<JwtAuthTokenClaims>(token_value, &decoding_key, &validation)
            .map_err(|e| {
                AuthTokenVerifyError::Backend(Box::new(JwtAuthTokenVerifierError::Decode(e)))
            })?;

        let jwt_claims = token_data.claims;
        jwt_claims
            .try_into_auth_token_claims()
            .map_err(|source| AuthTokenVerifyError::Backend(Box::new(source)))
    }
}
