use super::{OidcIssuerUrl, OidcProviderMetadata, OidcProviderMetadataSourceError};

#[allow(async_fn_in_trait)]
pub trait OidcProviderMetadataSource: Send + Sync {
    async fn read_provider_metadata(
        &self,
        issuer_url: &OidcIssuerUrl,
    ) -> Result<OidcProviderMetadata, OidcProviderMetadataSourceError>;
}
