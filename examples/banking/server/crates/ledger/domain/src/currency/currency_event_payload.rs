use appletheia::event_payload;

use crate::core::CurrencyAmount;

use super::{
    CurrencyDecimals, CurrencyEventPayloadError, CurrencyId, CurrencyName, CurrencyOwner,
    CurrencySymbol,
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
    },
    OwnershipTransferred {
        owner: CurrencyOwner,
    },
    SymbolChanged {
        symbol: CurrencySymbol,
    },
    NameChanged {
        name: CurrencyName,
    },
    SupplyIncreased {
        amount: CurrencyAmount,
    },
    SupplyDecreased {
        amount: CurrencyAmount,
    },
    Activated,
    Deactivated,
    Removed,
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
            CurrencyEventPayload::SYMBOL_CHANGED,
            appletheia::domain::EventName::new("symbol_changed")
        );
        assert_eq!(
            CurrencyEventPayload::NAME_CHANGED,
            appletheia::domain::EventName::new("name_changed")
        );
        assert_eq!(
            CurrencyEventPayload::SUPPLY_INCREASED,
            appletheia::domain::EventName::new("supply_increased")
        );
        assert_eq!(
            CurrencyEventPayload::SUPPLY_DECREASED,
            appletheia::domain::EventName::new("supply_decreased")
        );
        assert_eq!(
            CurrencyEventPayload::ACTIVATED,
            appletheia::domain::EventName::new("activated")
        );
        assert_eq!(
            CurrencyEventPayload::DEACTIVATED,
            appletheia::domain::EventName::new("deactivated")
        );
        assert_eq!(
            CurrencyEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
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
