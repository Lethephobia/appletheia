use serde::{Deserialize, Serialize};

use super::{MintMetadataDescription, MintMetadataImage, MintMetadataName, MintMetadataSymbol};

/// Off-chain metadata document for an on-chain mint account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintMetadataDocument {
    name: MintMetadataName,
    symbol: MintMetadataSymbol,
    description: Option<MintMetadataDescription>,
    image: Option<MintMetadataImage>,
}

impl MintMetadataDocument {
    pub fn new(
        name: MintMetadataName,
        symbol: MintMetadataSymbol,
        description: Option<MintMetadataDescription>,
        image: Option<MintMetadataImage>,
    ) -> Self {
        Self {
            name,
            symbol,
            description,
            image,
        }
    }

    pub fn name(&self) -> &MintMetadataName {
        &self.name
    }

    pub fn symbol(&self) -> &MintMetadataSymbol {
        &self.symbol
    }

    pub fn description(&self) -> Option<&MintMetadataDescription> {
        self.description.as_ref()
    }

    pub fn image(&self) -> Option<&MintMetadataImage> {
        self.image.as_ref()
    }
}
