use appletheia::event_payload;

use crate::core::TokenOwnerAddress;

use super::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkEventPayloadError,
    WalletBookmarkOwner,
};

/// Represents the domain events emitted by a `WalletBookmark` aggregate.
#[event_payload(error = WalletBookmarkEventPayloadError)]
pub enum WalletBookmarkEventPayload {
    Registered {
        owner: WalletBookmarkOwner,
        token_owner_address: TokenOwnerAddress,
    },
    DisplayNameSet {
        display_name: Option<WalletBookmarkDisplayName>,
    },
    DescriptionSet {
        description: Option<WalletBookmarkDescription>,
    },
    Removed,
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;
    use banking_iam_domain::UserId;

    use crate::core::{SolanaAccountAddress, SolanaTokenAccountOwnerAddress, TokenOwnerAddress};

    use super::{WalletBookmarkEventPayload, WalletBookmarkOwner};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            WalletBookmarkEventPayload::REGISTERED,
            appletheia::domain::EventName::new("registered")
        );
        assert_eq!(
            WalletBookmarkEventPayload::DISPLAY_NAME_SET,
            appletheia::domain::EventName::new("display_name_set")
        );
        assert_eq!(
            WalletBookmarkEventPayload::DESCRIPTION_SET,
            appletheia::domain::EventName::new("description_set")
        );
        assert_eq!(
            WalletBookmarkEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
        );
    }

    #[test]
    fn payload_name_matches_variant() {
        let payload = WalletBookmarkEventPayload::Removed;

        assert_eq!(payload.name(), WalletBookmarkEventPayload::REMOVED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = WalletBookmarkEventPayload::Registered {
            owner: WalletBookmarkOwner::User(UserId::new()),
            token_owner_address: TokenOwnerAddress::Solana(SolanaTokenAccountOwnerAddress::new(
                SolanaAccountAddress::try_from("11111111111111111111111111111111")
                    .expect("address should be valid"),
            )),
        };

        let value = payload
            .try_into_json_value()
            .expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("registered"));
    }
}
