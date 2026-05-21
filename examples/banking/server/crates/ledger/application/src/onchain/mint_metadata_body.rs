use serde::Serialize;

use super::MintMetadataDocument;

/// JSON body uploaded as off-chain mint metadata.
#[derive(Serialize)]
pub(super) struct MintMetadataBody {
    name: String,
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
}

impl MintMetadataBody {
    pub(super) fn new(
        name: String,
        symbol: String,
        description: Option<String>,
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

impl From<&MintMetadataDocument> for MintMetadataBody {
    fn from(document: &MintMetadataDocument) -> Self {
        Self::new(
            document.name().value().to_owned(),
            document.symbol().value().to_owned(),
            document.description().map(|value| value.value().to_owned()),
            document.image().map(ToString::to_string),
        )
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::currency::CurrencyImageUrl;
    use serde_json::json;

    use super::MintMetadataBody;
    use crate::onchain::{
        MintMetadataDescription, MintMetadataDocument, MintMetadataImageUri, MintMetadataName,
        MintMetadataSymbol,
    };

    #[test]
    fn converts_document_into_serializable_body() {
        let document = MintMetadataDocument::new(
            MintMetadataName::try_from("USD Coin").expect("name should be valid"),
            MintMetadataSymbol::try_from("USDC").expect("symbol should be valid"),
            Some(
                MintMetadataDescription::try_from("Stablecoin backed by USD")
                    .expect("description should be valid"),
            ),
            Some(
                MintMetadataImageUri::try_from(
                    CurrencyImageUrl::try_from(
                        "https://assets.example.com/currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002",
                    )
                    .expect("image URL should be valid"),
                )
                .expect("image URI should be valid"),
            ),
        );

        let body = MintMetadataBody::from(&document);

        let json: serde_json::Value =
            serde_json::to_value(&body).expect("body should serialize to JSON");
        assert_eq!(
            json,
            json!({
                "name": "USD Coin",
                "symbol": "USDC",
                "description": "Stablecoin backed by USD",
                "image": "https://assets.example.com/currencies/00000000-0000-0000-0000-000000000001/images/00000000-0000-0000-0000-000000000002"
            })
        );
    }
}
