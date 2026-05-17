use serde::Serialize;

/// JSON body uploaded as off-chain mint metadata.
#[derive(Serialize)]
pub(super) struct MintMetadataBody<'a> {
    name: &'a str,
    symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
}

impl<'a> MintMetadataBody<'a> {
    pub(super) fn new(
        name: &'a str,
        symbol: &'a str,
        description: Option<&'a str>,
        image: Option<String>,
    ) -> Self {
        Self {
            name,
            symbol,
            description,
            image,
        }
    }
}
