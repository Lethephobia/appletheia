use crate::banking_ledger::MintMetadataImagePublicBaseUrl;

/// Configuration for `CurrencyMintAccountMetadataSyncCommandHandler`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrencyMintAccountMetadataSyncCommandHandlerConfig {
    image_public_base_url: MintMetadataImagePublicBaseUrl,
}

impl CurrencyMintAccountMetadataSyncCommandHandlerConfig {
    pub fn new(image_public_base_url: MintMetadataImagePublicBaseUrl) -> Self {
        Self {
            image_public_base_url,
        }
    }

    pub fn image_public_base_url(&self) -> &MintMetadataImagePublicBaseUrl {
        &self.image_public_base_url
    }
}
