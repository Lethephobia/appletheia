use appletheia::event_payload;
use banking_iam_domain::UserId;

use crate::currency_definition::CurrencyDefinitionId;

use super::{AccountBalance, AccountEventPayloadError, AccountId};

/// Represents the domain events emitted by an `Account` aggregate.
#[event_payload(error = AccountEventPayloadError)]
pub enum AccountEventPayload {
    Opened {
        id: AccountId,
        user_id: UserId,
        currency_definition_id: CurrencyDefinitionId,
    },
    Frozen,
    Thawed,
    Closed,
    Deposited {
        amount: AccountBalance,
    },
    Withdrawn {
        amount: AccountBalance,
    },
    FundsReserved {
        amount: AccountBalance,
    },
    ReservedFundsReleased {
        amount: AccountBalance,
    },
    ReservedFundsCommitted {
        amount: AccountBalance,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use banking_iam_domain::UserId;

    use crate::currency_definition::CurrencyDefinitionId;

    use super::{AccountBalance, AccountEventPayload, AccountId};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            AccountEventPayload::OPENED,
            appletheia::domain::EventName::new("opened")
        );
        assert_eq!(
            AccountEventPayload::FROZEN,
            appletheia::domain::EventName::new("frozen")
        );
        assert_eq!(
            AccountEventPayload::THAWED,
            appletheia::domain::EventName::new("thawed")
        );
        assert_eq!(
            AccountEventPayload::CLOSED,
            appletheia::domain::EventName::new("closed")
        );
        assert_eq!(
            AccountEventPayload::DEPOSITED,
            appletheia::domain::EventName::new("deposited")
        );
        assert_eq!(
            AccountEventPayload::WITHDRAWN,
            appletheia::domain::EventName::new("withdrawn")
        );
        assert_eq!(
            AccountEventPayload::FUNDS_RESERVED,
            appletheia::domain::EventName::new("funds_reserved")
        );
        assert_eq!(
            AccountEventPayload::RESERVED_FUNDS_RELEASED,
            appletheia::domain::EventName::new("reserved_funds_released")
        );
        assert_eq!(
            AccountEventPayload::RESERVED_FUNDS_COMMITTED,
            appletheia::domain::EventName::new("reserved_funds_committed")
        );
    }

    #[test]
    fn payload_name_matches_variant() {
        let payload = AccountEventPayload::Frozen;

        assert_eq!(payload.name(), AccountEventPayload::FROZEN);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = AccountEventPayload::Opened {
            id: AccountId::new(),
            user_id: UserId::new(),
            currency_definition_id: CurrencyDefinitionId::new(),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("opened"));
    }

    #[test]
    fn serializes_balance_movement_payload_to_json() {
        let payload = AccountEventPayload::Deposited {
            amount: AccountBalance::new(10),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("deposited"));
    }

    #[test]
    fn serializes_reserved_funds_payload_to_json() {
        let payload = AccountEventPayload::FundsReserved {
            amount: AccountBalance::new(10),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("funds_reserved"));
    }
}
