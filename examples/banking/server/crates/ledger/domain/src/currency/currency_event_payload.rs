use appletheia::event_payload;

use crate::core::CurrencyAmount;

use super::{
    CurrencyActivateRejectionReason, CurrencyDeactivateRejectionReason, CurrencyDecimals,
    CurrencyDescription, CurrencyDescriptionChangeRejectionReason, CurrencyEventPayloadError,
    CurrencyImageChangeRejectionReason, CurrencyImageRef, CurrencyName,
    CurrencyNameChangeRejectionReason, CurrencyOwner, CurrencyOwnershipTransferRejectionReason,
    CurrencyProvisionRejectionReason, CurrencyRemoveRejectionReason,
    CurrencySupplyCommitRejectionReason, CurrencySupplyReleaseRejectionReason,
    CurrencySupplyReserveRejectionReason, CurrencySymbol, CurrencySymbolChangeRejectionReason,
    MintAccount, MintMetadataSyncRejectionReason,
};

/// Represents the domain events emitted by a `Currency` aggregate.
#[event_payload(error = CurrencyEventPayloadError)]
pub enum CurrencyEventPayload {
    Defined {
        owner: CurrencyOwner,
        symbol: CurrencySymbol,
        name: CurrencyName,
        decimals: CurrencyDecimals,
        description: Option<CurrencyDescription>,
        image: Option<CurrencyImageRef>,
    },
    Provisioned {
        mint_account: MintAccount,
    },
    ProvisionRejected {
        mint_account: Option<MintAccount>,
        reason: CurrencyProvisionRejectionReason,
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
    MintMetadataSynced,
    MintMetadataSyncRejected {
        reason: MintMetadataSyncRejectionReason,
    },
    SupplyReserved {
        amount: CurrencyAmount,
    },
    SupplyReserveRejected {
        amount: CurrencyAmount,
        reason: CurrencySupplyReserveRejectionReason,
    },
    MintSupplySynced {
        supply: CurrencyAmount,
    },
    SupplyCommitted {
        amount: CurrencyAmount,
    },
    SupplyCommitRejected {
        amount: CurrencyAmount,
        reason: CurrencySupplyCommitRejectionReason,
    },
    SupplyReleased {
        amount: CurrencyAmount,
    },
    SupplyReleaseRejected {
        amount: CurrencyAmount,
        reason: CurrencySupplyReleaseRejectionReason,
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
            CurrencyEventPayload::PROVISIONED,
            appletheia::domain::EventName::new("provisioned")
        );
        assert_eq!(
            CurrencyEventPayload::PROVISION_REJECTED,
            appletheia::domain::EventName::new("provision_rejected")
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
            CurrencyEventPayload::MINT_METADATA_SYNCED,
            appletheia::domain::EventName::new("mint_metadata_synced")
        );
        assert_eq!(
            CurrencyEventPayload::MINT_METADATA_SYNC_REJECTED,
            appletheia::domain::EventName::new("mint_metadata_sync_rejected")
        );
        assert_eq!(
            CurrencyEventPayload::SUPPLY_RESERVED,
            appletheia::domain::EventName::new("supply_reserved")
        );
        assert_eq!(
            CurrencyEventPayload::SUPPLY_RESERVE_REJECTED,
            appletheia::domain::EventName::new("supply_reserve_rejected")
        );
        assert_eq!(
            CurrencyEventPayload::MINT_SUPPLY_SYNCED,
            appletheia::domain::EventName::new("mint_supply_synced")
        );
        assert_eq!(
            CurrencyEventPayload::SUPPLY_COMMITTED,
            appletheia::domain::EventName::new("supply_committed")
        );
        assert_eq!(
            CurrencyEventPayload::SUPPLY_COMMIT_REJECTED,
            appletheia::domain::EventName::new("supply_commit_rejected")
        );
        assert_eq!(
            CurrencyEventPayload::SUPPLY_RELEASED,
            appletheia::domain::EventName::new("supply_released")
        );
        assert_eq!(
            CurrencyEventPayload::SUPPLY_RELEASE_REJECTED,
            appletheia::domain::EventName::new("supply_release_rejected")
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
