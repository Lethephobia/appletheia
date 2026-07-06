use appletheia::event_payload;

use crate::core::TokenAccountOwnerAddress;

use super::{
    WalletBookmarkDescription, WalletBookmarkDescriptionChangeRejectionReason,
    WalletBookmarkDisplayName, WalletBookmarkDisplayNameChangeRejectionReason,
    WalletBookmarkEventPayloadError, WalletBookmarkId, WalletBookmarkOwner,
    WalletBookmarkRemoveRejectionReason,
};

/// Represents the domain events emitted by a `WalletBookmark` aggregate.
#[event_payload(error = WalletBookmarkEventPayloadError)]
pub enum WalletBookmarkEventPayload {
    Registered {
        id: WalletBookmarkId,
        owner: WalletBookmarkOwner,
        display_name: Option<WalletBookmarkDisplayName>,
        description: Option<WalletBookmarkDescription>,
        token_account_owner_address: TokenAccountOwnerAddress,
    },
    Removed,
    RemoveRejected {
        reason: WalletBookmarkRemoveRejectionReason,
    },
    DisplayNameChanged {
        display_name: Option<WalletBookmarkDisplayName>,
    },
    DisplayNameChangeRejected {
        display_name: Option<WalletBookmarkDisplayName>,
        reason: WalletBookmarkDisplayNameChangeRejectionReason,
    },
    DescriptionChanged {
        description: Option<WalletBookmarkDescription>,
    },
    DescriptionChangeRejected {
        description: Option<WalletBookmarkDescription>,
        reason: WalletBookmarkDescriptionChangeRejectionReason,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;
    use banking_iam_domain::UserId;

    use crate::core::TokenAccountOwnerAddress;

    use super::{
        WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkEventPayload,
        WalletBookmarkId, WalletBookmarkOwner,
    };

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            WalletBookmarkEventPayload::REGISTERED,
            appletheia::domain::EventName::new("registered")
        );
        assert_eq!(
            WalletBookmarkEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
        );
        assert_eq!(
            WalletBookmarkEventPayload::REMOVE_REJECTED,
            appletheia::domain::EventName::new("remove_rejected")
        );
        assert_eq!(
            WalletBookmarkEventPayload::DISPLAY_NAME_CHANGED,
            appletheia::domain::EventName::new("display_name_changed")
        );
        assert_eq!(
            WalletBookmarkEventPayload::DISPLAY_NAME_CHANGE_REJECTED,
            appletheia::domain::EventName::new("display_name_change_rejected")
        );
        assert_eq!(
            WalletBookmarkEventPayload::DESCRIPTION_CHANGED,
            appletheia::domain::EventName::new("description_changed")
        );
        assert_eq!(
            WalletBookmarkEventPayload::DESCRIPTION_CHANGE_REJECTED,
            appletheia::domain::EventName::new("description_change_rejected")
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
            id: WalletBookmarkId::new(),
            owner: WalletBookmarkOwner::User(UserId::new()),
            display_name: Some(
                WalletBookmarkDisplayName::try_from("Main wallet")
                    .expect("display name should be valid"),
            ),
            description: Some(
                WalletBookmarkDescription::try_from("Personal main wallet")
                    .expect("description should be valid"),
            ),
            token_account_owner_address: TokenAccountOwnerAddress::try_from(
                "11111111111111111111111111111111",
            )
            .expect("address should be valid"),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("registered"));
    }
}
