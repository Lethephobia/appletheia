use crate::jwt::{JwkKeyId, JwksSource};
use appletheia_application::authentication::oidc::{
    OidcIdToken, OidcIdTokenClaims, OidcIdTokenVerifier, OidcIdTokenVerifyContext,
    OidcIdTokenVerifyError, OidcNonce,
};
use base64::Engine;
use chrono::{DateTime, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use sha2::{Digest, Sha256, Sha384, Sha512};

use super::jwt_oidc_id_token_claims::JwtOidcIdTokenClaims;
use super::jwt_oidc_id_token_verifier_config::JwtOidcIdTokenVerifierConfig;

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
    ) -> Result<(), OidcIdTokenVerifyError> {
        let Some(expected) = expected else {
            return Ok(());
        };
        let Some(actual) = actual else {
            return Err(OidcIdTokenVerifyError::InvalidIdToken);
        };
        if expected.value() != actual {
            return Err(OidcIdTokenVerifyError::InvalidIdToken);
        }
        Ok(())
    }

    fn validate_issued_at(
        now: DateTime<Utc>,
        leeway_seconds: u64,
        issued_at_seconds: Option<u64>,
    ) -> Result<(), OidcIdTokenVerifyError> {
        let Some(issued_at_seconds) = issued_at_seconds else {
            return Ok(());
        };
        let now_seconds =
            u64::try_from(now.timestamp()).map_err(|_| OidcIdTokenVerifyError::InvalidIdToken)?;
        if issued_at_seconds > now_seconds.saturating_add(leeway_seconds) {
            return Err(OidcIdTokenVerifyError::InvalidIdToken);
        }
        Ok(())
    }

    fn compute_access_token_hash(
        access_token: &str,
        algorithm: Algorithm,
    ) -> Result<String, OidcIdTokenVerifyError> {
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
            _ => return Err(OidcIdTokenVerifyError::InvalidIdToken),
        };

        let half_len = digest.len() / 2;
        Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&digest[..half_len]))
    }

    fn validate_access_token_hash(
        access_token: Option<&str>,
        access_token_hash: Option<&str>,
        algorithm: Algorithm,
    ) -> Result<(), OidcIdTokenVerifyError> {
        let Some(access_token) = access_token else {
            return Ok(());
        };
        let Some(access_token_hash) = access_token_hash else {
            return Ok(());
        };

        let computed = Self::compute_access_token_hash(access_token, algorithm)?;
        if computed != access_token_hash {
            return Err(OidcIdTokenVerifyError::InvalidIdToken);
        }
        Ok(())
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
    ) -> Result<OidcIdTokenClaims, OidcIdTokenVerifyError> {
        let token_value = id_token.value();

        let header =
            decode_header(token_value).map_err(|_| OidcIdTokenVerifyError::InvalidIdToken)?;

        let key_id = header.kid.ok_or(OidcIdTokenVerifyError::InvalidIdToken)?;
        let key_id = JwkKeyId::new(key_id).map_err(|_| OidcIdTokenVerifyError::InvalidIdToken)?;

        let jwks = self
            .jwks_source
            .read_jwks(&context.jwks_uri)
            .await
            .map_err(|source| OidcIdTokenVerifyError::Backend(Box::new(source)))?;

        let jwk = jwks
            .find(key_id.value())
            .ok_or(OidcIdTokenVerifyError::InvalidIdToken)?;

        let decoding_key =
            DecodingKey::from_jwk(jwk).map_err(|e| OidcIdTokenVerifyError::Backend(Box::new(e)))?;

        let validation = self.validation(&context);
        let token_data = decode::<JwtOidcIdTokenClaims>(token_value, &decoding_key, &validation)
            .map_err(|_| OidcIdTokenVerifyError::InvalidIdToken)?;
        let jwt_claims = token_data.claims;

        let now = Utc::now();
        Self::validate_issued_at(
            now,
            self.config.leeway_seconds().value(),
            jwt_claims.issued_at,
        )?;

        Self::validate_nonce(context.expected_nonce.as_ref(), jwt_claims.nonce.as_deref())?;

        Self::validate_access_token_hash(
            context.access_token.as_ref().map(|token| token.value()),
            jwt_claims.access_token_hash.as_deref(),
            header.alg,
        )?;

        jwt_claims.try_into_id_token_claims()
    }
}
