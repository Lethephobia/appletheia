use appletheia::event_payload;

use crate::core::{CurrencyDecimals, CurrencySymbol};

use super::{CurrencyDefinitionEventPayloadError, CurrencyDefinitionId, CurrencyDefinitionName};

/// Represents the domain events emitted by a `CurrencyDefinition` aggregate.
#[event_payload(error = CurrencyDefinitionEventPayloadError)]
pub enum CurrencyDefinitionEventPayload {
    Defined {
        id: CurrencyDefinitionId,
        symbol: CurrencySymbol,
        name: CurrencyDefinitionName,
        decimals: CurrencyDecimals,
    },
    SymbolChanged {
        symbol: CurrencySymbol,
    },
    NameChanged {
        name: CurrencyDefinitionName,
    },
    Activated,
    Deactivated,
    Removed,
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use crate::core::{CurrencyDecimals, CurrencySymbol};

    use super::{CurrencyDefinitionEventPayload, CurrencyDefinitionId, CurrencyDefinitionName};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            CurrencyDefinitionEventPayload::DEFINED,
            appletheia::domain::EventName::new("defined")
        );
        assert_eq!(
            CurrencyDefinitionEventPayload::SYMBOL_CHANGED,
            appletheia::domain::EventName::new("symbol_changed")
        );
        assert_eq!(
            CurrencyDefinitionEventPayload::NAME_CHANGED,
            appletheia::domain::EventName::new("name_changed")
        );
        assert_eq!(
            CurrencyDefinitionEventPayload::ACTIVATED,
            appletheia::domain::EventName::new("activated")
        );
        assert_eq!(
            CurrencyDefinitionEventPayload::DEACTIVATED,
            appletheia::domain::EventName::new("deactivated")
        );
        assert_eq!(
            CurrencyDefinitionEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
        );
    }

    #[test]
    fn payload_name_matches_variant() {
        let payload = CurrencyDefinitionEventPayload::Activated;

        assert_eq!(payload.name(), CurrencyDefinitionEventPayload::ACTIVATED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = CurrencyDefinitionEventPayload::Defined {
            id: CurrencyDefinitionId::new(),
            symbol: CurrencySymbol::try_from("usdc").expect("symbol should be valid"),
            name: CurrencyDefinitionName::try_from("USD Coin").expect("name should be valid"),
            decimals: CurrencyDecimals::new(6),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("defined"));
    }
}
