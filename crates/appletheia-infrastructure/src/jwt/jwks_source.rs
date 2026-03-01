use appletheia_application::authentication::oidc::OidcJwksUri;

use crate::jwt::{Jwks, JwksSourceError};

#[allow(async_fn_in_trait)]
pub trait JwksSource: Send + Sync {
    async fn read_jwks(&self, jwks_uri: &OidcJwksUri) -> Result<Jwks, JwksSourceError>;
}
