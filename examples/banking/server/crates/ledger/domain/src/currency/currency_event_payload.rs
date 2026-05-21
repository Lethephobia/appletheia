use appletheia::event_payload;

use crate::core::CurrencyAmount;

use super::{
    CurrencyActivateRejectionReason, CurrencyDeactivateRejectionReason, CurrencyDecimals,
    CurrencyDescription, CurrencyDescriptionChangeRejectionReason, CurrencyEventPayloadError,
    CurrencyId, CurrencyImageChangeRejectionReason, CurrencyImageRef, CurrencyMintAccount,
    CurrencyMintAccountCreationRequestRejectionReason, CurrencyMintAccountRecordRejectionReason,
    CurrencyName, CurrencyNameChangeRejectionReason, CurrencyOwner,
    CurrencyOwnershipTransferRejectionReason, CurrencyRemoveRejectionReason,
    CurrencySupplyDecreaseRejectionReason, CurrencySupplyIncreaseRejectionReason, CurrencySymbol,
    CurrencySymbolChangeRejectionReason,
};

/// Represents the domain events emitted by a `Currency` aggregate.
#[event_payload(error = CurrencyEventPayloadError)]
pub enum CurrencyEventPayload {
    Defined {
        id: CurrencyId,
        owner: CurrencyOwner,
        symbol: CurrencySymbol,
        name: CurrencyName,
        decimals: CurrencyDecimals,
        description: Option<CurrencyDescription>,
        image: Option<CurrencyImageRef>,
    },
    OwnershipTransferred {
        owner: CurrencyOwner,
    },
    OwnershipTransferRejected {
        owner: CurrencyOwner,
        reason: CurrencyOwnershipTransferRejectionReason,
    },
    SymbolChanged {
        symbol: CurrencySymbol,
    },
    SymbolChangeRejected {
        symbol: CurrencySymbol,
        reason: CurrencySymbolChangeRejectionReason,
    },
    NameChanged {
        name: CurrencyName,
    },
    NameChangeRejected {
        name: CurrencyName,
        reason: CurrencyNameChangeRejectionReason,
    },
    DescriptionChanged {
        description: Option<CurrencyDescription>,
    },
    DescriptionChangeRejected {
        description: Option<CurrencyDescription>,
        reason: CurrencyDescriptionChangeRejectionReason,
    },
    ImageChanged {
        image: Option<CurrencyImageRef>,
        old_image: Option<CurrencyImageRef>,
    },
    ImageChangeRejected {
        image: Option<CurrencyImageRef>,
        reason: CurrencyImageChangeRejectionReason,
    },
    MintAccountCreationRequested,
    MintAccountCreationRequestRejected {
        reason: CurrencyMintAccountCreationRequestRejectionReason,
    },
    MintAccountRecorded {
        mint_account: CurrencyMintAccount,
    },
    MintAccountRecordRejected {
        mint_account: Option<CurrencyMintAccount>,
        reason: CurrencyMintAccountRecordRejectionReason,
    },
    SupplyIncreased {
        amount: CurrencyAmount,
    },
    SupplyIncreaseRejected {
        amount: CurrencyAmount,
        reason: CurrencySupplyIncreaseRejectionReason,
    },
    SupplyDecreased {
        amount: CurrencyAmount,
    },
    SupplyDecreaseRejected {
        amount: CurrencyAmount,
        reason: CurrencySupplyDecreaseRejectionReason,
    },
    Activated,
    ActivateRejected {
        reason: CurrencyActivateRejectionReason,
    },
    Deactivated,
    DeactivateRejected {
        reason: CurrencyDeactivateRejectionReason,
    },
    Removed,
    RemoveRejected {
        reason: CurrencyRemoveRejectionReason,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use banking_iam_domain::UserId;

    use super::{CurrencyEventPayload, CurrencyOwner};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            CurrencyEventPayload::DEFINED,
            appletheia::domain::EventName::new("defined")
        );
        assert_eq!(
            CurrencyEventPayload::OWNERSHIP_TRANSFERRED,
            appletheia::domain::EventName::new("ownership_transferred")
        );
        assert_eq!(
            CurrencyEventPayload::OWNERSHIP_TRANSFER_REJECTED,
            appletheia::domain::EventName::new("ownership_transfer_rejected")
        );
        assert_eq!(
            CurrencyEventPayload::SYMBOL_CHANGED,
            appletheia::domain::EventName::new("symbol_changed")
        );
        assert_eq!(
            CurrencyEventPayload::SYMBOL_CHANGE_REJECTED,
            appletheia::domain::EventName::new("symbol_change_rejected")
        );
        assert_eq!(
            CurrencyEventPayload::NAME_CHANGED,
            appletheia::domain::EventName::new("name_changed")
        );
        assert_eq!(
            CurrencyEventPayload::NAME_CHANGE_REJECTED,
            appletheia::domain::EventName::new("name_change_rejected")
        );
        assert_eq!(
            CurrencyEventPayload::DESCRIPTION_CHANGED,
            appletheia::domain::EventName::new("description_changed")
        );
        assert_eq!(
            CurrencyEventPayload::DESCRIPTION_CHANGE_REJECTED,
            appletheia::domain::EventName::new("description_change_rejected")
        );
        assert_eq!(
            CurrencyEventPayload::IMAGE_CHANGED,
            appletheia::domain::EventName::new("image_changed")
        );
        assert_eq!(
            CurrencyEventPayload::IMAGE_CHANGE_REJECTED,
            appletheia::domain::EventName::new("image_change_rejected")
        );
        assert_eq!(
            CurrencyEventPayload::MINT_ACCOUNT_CREATION_REQUESTED,
            appletheia::domain::EventName::new("mint_account_creation_requested")
        );
        assert_eq!(
            CurrencyEventPayload::MINT_ACCOUNT_CREATION_REQUEST_REJECTED,
            appletheia::domain::EventName::new("mint_account_creation_request_rejected")
        );
        assert_eq!(
            CurrencyEventPayload::MINT_ACCOUNT_RECORDED,
            appletheia::domain::EventName::new("mint_account_recorded")
        );
        assert_eq!(
            CurrencyEventPayload::MINT_ACCOUNT_RECORD_REJECTED,
            appletheia::domain::EventName::new("mint_account_record_rejected")
        );
        assert_eq!(
            CurrencyEventPayload::SUPPLY_INCREASED,
            appletheia::domain::EventName::new("supply_increased")
        );
        assert_eq!(
            CurrencyEventPayload::SUPPLY_INCREASE_REJECTED,
            appletheia::domain::EventName::new("supply_increase_rejected")
        );
        assert_eq!(
            CurrencyEventPayload::SUPPLY_DECREASED,
            appletheia::domain::EventName::new("supply_decreased")
        );
        assert_eq!(
            CurrencyEventPayload::SUPPLY_DECREASE_REJECTED,
            appletheia::domain::EventName::new("supply_decrease_rejected")
        );
        assert_eq!(
            CurrencyEventPayload::ACTIVATED,
            appletheia::domain::EventName::new("activated")
        );
        assert_eq!(
            CurrencyEventPayload::ACTIVATE_REJECTED,
            appletheia::domain::EventName::new("activate_rejected")
        );
        assert_eq!(
            CurrencyEventPayload::DEACTIVATED,
            appletheia::domain::EventName::new("deactivated")
        );
        assert_eq!(
            CurrencyEventPayload::DEACTIVATE_REJECTED,
            appletheia::domain::EventName::new("deactivate_rejected")
        );
        assert_eq!(
            CurrencyEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
        );
        assert_eq!(
            CurrencyEventPayload::REMOVE_REJECTED,
            appletheia::domain::EventName::new("remove_rejected")
        );
    }

    #[test]
    fn payload_name_matches_variant() {
        let payload = CurrencyEventPayload::Activated;

        assert_eq!(payload.name(), CurrencyEventPayload::ACTIVATED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = CurrencyEventPayload::Defined {
            id: super::CurrencyId::new(),
            owner: CurrencyOwner::User(UserId::new()),
            symbol: super::CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            name: super::CurrencyName::try_from("USD Coin").expect("name should be valid"),
            decimals: super::CurrencyDecimals::new(6),
            description: None,
            image: None,
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("defined"));
        assert_eq!(value["data"]["owner"]["type"], serde_json::json!("user"));
    }

    #[test]
    fn serializes_ownership_transferred_payload_to_json() {
        let payload = CurrencyEventPayload::OwnershipTransferred {
            owner: CurrencyOwner::User(UserId::new()),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("ownership_transferred"));
        assert_eq!(value["data"]["owner"]["type"], serde_json::json!("user"));
    }
}
