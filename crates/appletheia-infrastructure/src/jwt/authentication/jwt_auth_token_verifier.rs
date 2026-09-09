use appletheia_application::{
    AuthToken, AuthTokenClaims, AuthTokenVerifier, AuthTokenVerifierError,
};
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

    fn validation(&self, decoding_key: &DecodingKey) -> Validation {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.algorithms = [Algorithm::RS256, Algorithm::EdDSA]
            .into_iter()
            .filter(|algorithm| algorithm.family() == decoding_key.family())
            .collect();
        validation.leeway = self.config.leeway_seconds().value();
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);

        let issuers: Vec<String> = self
            .config
            .allowed_issuer_urls()
            .values()
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
    async fn verify(&self, token: &AuthToken) -> Result<AuthTokenClaims, AuthTokenVerifierError> {
        let token_value = token.value();

        let header = decode_header(token_value).map_err(|e| {
            AuthTokenVerifierError::Backend(Box::new(JwtAuthTokenVerifierError::DecodeHeader(e)))
        })?;

        let key_id = header.kid.ok_or_else(|| {
            AuthTokenVerifierError::Backend(Box::new(JwtAuthTokenVerifierError::MissingKeyId))
        })?;

        let jwk = self.config.jwks().find(&key_id).ok_or_else(|| {
            AuthTokenVerifierError::Backend(Box::new(JwtAuthTokenVerifierError::UnknownKeyId))
        })?;

        let decoding_key = DecodingKey::from_jwk(jwk).map_err(|e| {
            AuthTokenVerifierError::Backend(Box::new(JwtAuthTokenVerifierError::InvalidKey(e)))
        })?;

        let validation = self.validation(&decoding_key);
        let token_data = decode::<JwtAuthTokenClaims>(token_value, &decoding_key, &validation)
            .map_err(|e| {
                AuthTokenVerifierError::Backend(Box::new(JwtAuthTokenVerifierError::Decode(e)))
            })?;

        let jwt_claims = token_data.claims;
        jwt_claims
            .try_into_auth_token_claims()
            .map_err(|source| AuthTokenVerifierError::Backend(Box::new(source)))
    }
}

#[cfg(test)]
mod tests {
    use appletheia_application::{
        AggregateIdValue, AggregateRef, AggregateTypeOwned, AuthTokenAudience, AuthTokenAudiences,
        AuthTokenExpiresIn, AuthTokenIssueRequest, AuthTokenIssuer, AuthTokenIssuerUrl,
        AuthTokenIssuerUrls,
    };
    use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
    use chrono::Duration;
    use jsonwebtoken::{
        EncodingKey, Header, dangerous::insecure_decode, encode, errors::ErrorKind,
    };
    use serde_json::{Value, json};
    use uuid::Uuid;

    use super::*;
    use crate::jwt::authentication::{JwtAuthTokenIssuer, JwtAuthTokenIssuerConfig};
    use crate::jwt::{JwkKeyId, Jwks, JwtSigningKey};

    #[tokio::test]
    async fn issued_token_verifies_and_invalid_signatures_and_claims_are_rejected() {
        // Public test key from RFC 8032, test vector 1; never use it outside tests.
        let signing_key = JwtSigningKey::EdDsa {
            key_id: JwkKeyId::new("test-key".to_owned()).unwrap(),
            private_key_pem: b"-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIJ1hsZ3v/VpguoRK9JLsLMREScVpezJpGXA7rAMcrn9g\n-----END PRIVATE KEY-----\n".to_vec(),
        };
        let issuer_url = "https://issuer.example.com/"
            .parse::<AuthTokenIssuerUrl>()
            .unwrap();
        let audiences =
            AuthTokenAudiences::new(vec![AuthTokenAudience::new("test-api".to_owned()).unwrap()])
                .unwrap();
        let issuer = JwtAuthTokenIssuer::new(JwtAuthTokenIssuerConfig::new(
            issuer_url.clone(),
            audiences.clone(),
            AuthTokenExpiresIn::new(Duration::minutes(5)).unwrap(),
            signing_key.clone(),
        ));
        let jwks = Jwks::try_from(
            r#"{"keys":[{"kty":"OKP","crv":"Ed25519","kid":"test-key","alg":"EdDSA","x":"11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo"}]}"#,
        )
        .unwrap();
        let verifier = JwtAuthTokenVerifier::new(JwtAuthTokenVerifierConfig::new(
            AuthTokenIssuerUrls::new(vec![issuer_url]).unwrap(),
            audiences,
            jwks,
        ));
        let subject = AggregateRef {
            aggregate_type: AggregateTypeOwned::new("subject".to_owned()).unwrap(),
            aggregate_id: AggregateIdValue::from(Uuid::from_u128(1)),
        };
        let issued = issuer
            .issue(AuthTokenIssueRequest::new(subject))
            .await
            .unwrap();
        let verified = verifier.verify(issued.token()).await.unwrap();
        assert_eq!(verified.subject(), issued.claims().subject());
        assert_eq!(verified.issuer_url(), issued.claims().issuer_url());
        assert_eq!(verified.audiences(), issued.claims().audiences());
        assert_eq!(verified.token_id(), issued.claims().token_id());
        assert_eq!(
            verified.issued_at().value().timestamp(),
            issued.claims().issued_at().value().timestamp()
        );
        assert_eq!(
            verified.expires_at().value().timestamp(),
            issued.claims().expires_at().value().timestamp()
        );

        let token_parts: Vec<&str> = issued.token().value().split('.').collect();
        let mut invalid_signature = token_parts[2].as_bytes().to_vec();
        invalid_signature[0] = if invalid_signature[0] == b'A' {
            b'B'
        } else {
            b'A'
        };
        let tampered = AuthToken::new(format!(
            "{}.{}.{}",
            token_parts[0],
            token_parts[1],
            String::from_utf8(invalid_signature).unwrap(),
        ));
        let signature_error = verifier.verify(&tampered).await.unwrap_err();
        assert_decode_error(signature_error, ErrorKind::InvalidSignature);

        let valid_claims: Value = insecure_decode::<Value>(issued.token().value())
            .unwrap()
            .claims;
        let encoding_key = signing_key.try_into_encoding_key().unwrap();
        let mut header = Header::new(Algorithm::EdDSA);
        header.kid = Some("test-key".to_owned());
        for (name, value, expected) in [
            (
                "iss",
                json!("https://other.example.com/"),
                ErrorKind::InvalidIssuer,
            ),
            ("aud", json!(["other-api"]), ErrorKind::InvalidAudience),
            ("exp", json!(1), ErrorKind::ExpiredSignature),
        ] {
            let mut invalid_claims = valid_claims.clone();
            invalid_claims[name] = value;
            let invalid_token =
                AuthToken::new(encode(&header, &invalid_claims, &encoding_key).unwrap());
            let claim_error = verifier.verify(&invalid_token).await.unwrap_err();
            assert_decode_error(claim_error, expected);
        }

        let hmac_secret = [42; 32];
        let hmac_jwks = serde_json::from_value(json!({
            "keys": [{
                "kty": "oct",
                "kid": "hmac-key",
                "alg": "HS256",
                "k": URL_SAFE_NO_PAD.encode(hmac_secret),
            }],
        }))
        .unwrap();
        let hmac_verifier = JwtAuthTokenVerifier::new(JwtAuthTokenVerifierConfig::new(
            verifier.config.allowed_issuer_urls().clone(),
            verifier.config.allowed_audiences().clone(),
            hmac_jwks,
        ));
        let mut hmac_header = Header::new(Algorithm::HS256);
        hmac_header.kid = Some("hmac-key".to_owned());
        let hmac_token = AuthToken::new(
            encode(
                &hmac_header,
                &valid_claims,
                &EncodingKey::from_secret(&hmac_secret),
            )
            .unwrap(),
        );
        let algorithm_error = hmac_verifier.verify(&hmac_token).await.unwrap_err();
        assert_decode_error(algorithm_error, ErrorKind::InvalidAlgorithm);
    }

    fn assert_decode_error(error: AuthTokenVerifierError, expected: ErrorKind) {
        let AuthTokenVerifierError::Backend(source) = error;
        let JwtAuthTokenVerifierError::Decode(decode_error) =
            source.downcast_ref::<JwtAuthTokenVerifierError>().unwrap()
        else {
            panic!("expected a JWT decode error, got {source}");
        };
        assert_eq!(decode_error.kind(), &expected);
    }
}
