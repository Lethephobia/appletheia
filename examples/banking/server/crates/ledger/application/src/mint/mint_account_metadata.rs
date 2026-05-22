use serde::{Deserialize, Serialize};

use super::{MintMetadataName, MintMetadataSymbol, MintMetadataUri};

/// Metadata written to the on-chain mint account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintAccountMetadata {
    name: MintMetadataName,
    symbol: MintMetadataSymbol,
    uri: MintMetadataUri,
}

impl MintAccountMetadata {
    pub fn new(name: MintMetadataName, symbol: MintMetadataSymbol, uri: MintMetadataUri) -> Self {
        Self { name, symbol, uri }
    }

    pub fn name(&self) -> &MintMetadataName {
        &self.name
    }

    pub fn symbol(&self) -> &MintMetadataSymbol {
        &self.symbol
    }

    pub fn uri(&self) -> &MintMetadataUri {
        &self.uri
    }
}
