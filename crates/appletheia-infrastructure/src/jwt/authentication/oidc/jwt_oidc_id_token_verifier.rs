use crate::jwt::{JwkKeyId, JwksSource};
use appletheia_application::authentication::oidc::{
    OidcIdToken, OidcIdTokenClaims, OidcIdTokenVerifier, OidcIdTokenVerifierError,
    OidcIdTokenVerifyContext, OidcNonce,
};
use base64::Engine;
use chrono::{DateTime, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use sha2::{Digest, Sha256, Sha384, Sha512};

use super::jwt_oidc_id_token_claims::JwtOidcIdTokenClaims;
use super::jwt_oidc_id_token_verifier_config::JwtOidcIdTokenVerifierConfig;
use super::jwt_oidc_id_token_verifier_error::JwtOidcIdTokenVerifierError;

#[derive(Clone, Debug)]
pub struct JwtOidcIdTokenVerifier<JS>
where
    JS: JwksSource,
{
    config: JwtOidcIdTokenVerifierConfig,
    jwks_source: JS,
}

impl<JS> JwtOidcIdTokenVerifier<JS>
where
    JS: JwksSource,
{
    pub fn new(config: JwtOidcIdTokenVerifierConfig, jwks_source: JS) -> Self {
        Self {
            config,
            jwks_source,
        }
    }

    fn validation(&self, context: &OidcIdTokenVerifyContext) -> Validation {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.algorithms = vec![Algorithm::RS256, Algorithm::EdDSA];
        validation.leeway = self.config.leeway_seconds().value();
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);

        validation.set_issuer(&[context.issuer_url.value().to_string()]);
        validation.set_audience(&[context.client_id.value().to_string()]);

        validation
    }

    fn validate_nonce(
        expected: Option<&OidcNonce>,
        actual: Option<&str>,
    ) -> Result<(), JwtOidcIdTokenVerifierError> {
        let Some(expected) = expected else {
            return Ok(());
        };
        let Some(actual) = actual else {
            return Err(JwtOidcIdTokenVerifierError::MissingNonce);
        };
        if expected.value() != actual {
            return Err(JwtOidcIdTokenVerifierError::NonceMismatch);
        }
        Ok(())
    }

    fn validate_issued_at(
        now: DateTime<Utc>,
        leeway_seconds: u64,
        issued_at_seconds: Option<u64>,
    ) -> Result<(), JwtOidcIdTokenVerifierError> {
        let Some(issued_at_seconds) = issued_at_seconds else {
            return Ok(());
        };
        let now_seconds = u64::try_from(now.timestamp())
            .map_err(|_| JwtOidcIdTokenVerifierError::InvalidCurrentTime)?;
        if issued_at_seconds > now_seconds.saturating_add(leeway_seconds) {
            return Err(JwtOidcIdTokenVerifierError::InvalidIssuedAt);
        }
        Ok(())
    }

    fn compute_access_token_hash(
        access_token: &str,
        algorithm: Algorithm,
    ) -> Result<String, JwtOidcIdTokenVerifierError> {
        let digest: Vec<u8> = match algorithm {
            Algorithm::RS256 | Algorithm::PS256 | Algorithm::ES256 => {
                let mut hasher = Sha256::new();
                hasher.update(access_token.as_bytes());
                hasher.finalize().to_vec()
            }
            Algorithm::RS384 | Algorithm::PS384 | Algorithm::ES384 => {
                let mut hasher = Sha384::new();
                hasher.update(access_token.as_bytes());
                hasher.finalize().to_vec()
            }
            Algorithm::RS512 | Algorithm::PS512 | Algorithm::EdDSA => {
                let mut hasher = Sha512::new();
                hasher.update(access_token.as_bytes());
                hasher.finalize().to_vec()
            }
            _ => return Err(JwtOidcIdTokenVerifierError::UnsupportedAlgorithm),
        };

        let half_len = digest.len() / 2;
        Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&digest[..half_len]))
    }

    fn validate_access_token_hash(
        access_token: Option<&str>,
        access_token_hash: Option<&str>,
        algorithm: Algorithm,
    ) -> Result<(), JwtOidcIdTokenVerifierError> {
        let Some(access_token) = access_token else {
            return Ok(());
        };
        let Some(access_token_hash) = access_token_hash else {
            return Ok(());
        };

        let computed = Self::compute_access_token_hash(access_token, algorithm)?;
        if computed != access_token_hash {
            return Err(JwtOidcIdTokenVerifierError::AccessTokenHashMismatch);
        }
        Ok(())
    }

    fn map_error(error: JwtOidcIdTokenVerifierError) -> OidcIdTokenVerifierError {
        match error {
            JwtOidcIdTokenVerifierError::JwksSource(_)
            | JwtOidcIdTokenVerifierError::InvalidKey(_) => {
                OidcIdTokenVerifierError::Backend(Box::new(error))
            }
            _ => OidcIdTokenVerifierError::invalid_id_token_with_source(error),
        }
    }
}

impl<JS> OidcIdTokenVerifier for JwtOidcIdTokenVerifier<JS>
where
    JS: JwksSource,
{
    async fn verify(
        &self,
        id_token: &OidcIdToken,
        context: OidcIdTokenVerifyContext,
    ) -> Result<OidcIdTokenClaims, OidcIdTokenVerifierError> {
        let token_value = id_token.value();

        let header = decode_header(token_value)
            .map_err(JwtOidcIdTokenVerifierError::DecodeHeader)
            .map_err(Self::map_error)?;

        let key_id = header
            .kid
            .ok_or(JwtOidcIdTokenVerifierError::MissingKeyId)
            .map_err(Self::map_error)?;
        let key_id = JwkKeyId::new(key_id)
            .map_err(JwtOidcIdTokenVerifierError::InvalidKeyId)
            .map_err(Self::map_error)?;

        let jwks = self
            .jwks_source
            .read_jwks(&context.jwks_uri)
            .await
            .map_err(|source| {
                OidcIdTokenVerifierError::Backend(Box::new(
                    JwtOidcIdTokenVerifierError::JwksSource(Box::new(source)),
                ))
            })?;

        let jwk = jwks
            .find(key_id.value())
            .ok_or(JwtOidcIdTokenVerifierError::UnknownKeyId)
            .map_err(Self::map_error)?;

        let decoding_key = DecodingKey::from_jwk(jwk)
            .map_err(JwtOidcIdTokenVerifierError::InvalidKey)
            .map_err(Self::map_error)?;

        let validation = self.validation(&context);
        let token_data = decode::<JwtOidcIdTokenClaims>(token_value, &decoding_key, &validation)
            .map_err(JwtOidcIdTokenVerifierError::Decode)
            .map_err(Self::map_error)?;
        let jwt_claims = token_data.claims;

        let now = Utc::now();
        Self::validate_issued_at(
            now,
            self.config.leeway_seconds().value(),
            jwt_claims.issued_at,
        )
        .map_err(Self::map_error)?;

        Self::validate_nonce(context.expected_nonce.as_ref(), jwt_claims.nonce.as_deref())
            .map_err(Self::map_error)?;

        Self::validate_access_token_hash(
            context.access_token.as_ref().map(|token| token.value()),
            jwt_claims.access_token_hash.as_deref(),
            header.alg,
        )
        .map_err(Self::map_error)?;

        jwt_claims
            .try_into_id_token_claims()
            .map_err(JwtOidcIdTokenVerifierError::from)
            .map_err(Self::map_error)
    }
}
