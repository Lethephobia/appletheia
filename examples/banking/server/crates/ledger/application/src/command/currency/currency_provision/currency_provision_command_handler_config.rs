use crate::mint::MintMetadataImagePublicBaseUrl;

/// Configuration for `CurrencyProvisionCommandHandler`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrencyProvisionCommandHandlerConfig {
    image_public_base_url: MintMetadataImagePublicBaseUrl,
}

impl CurrencyProvisionCommandHandlerConfig {
    pub fn new(image_public_base_url: MintMetadataImagePublicBaseUrl) -> Self {
        Self {
            image_public_base_url,
        }
    }

    pub fn image_public_base_url(&self) -> &MintMetadataImagePublicBaseUrl {
        &self.image_public_base_url
    }
}
