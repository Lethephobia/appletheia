use crate::onchain::MintMetadataImagePublicBaseUrl;

/// Configuration for `CurrencyMintAccountCreateCommandHandler`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrencyMintAccountCreateCommandHandlerConfig {
    image_public_base_url: MintMetadataImagePublicBaseUrl,
}

impl CurrencyMintAccountCreateCommandHandlerConfig {
    pub fn new(image_public_base_url: MintMetadataImagePublicBaseUrl) -> Self {
        Self {
            image_public_base_url,
        }
    }

    pub fn image_public_base_url(&self) -> &MintMetadataImagePublicBaseUrl {
        &self.image_public_base_url
    }
}
