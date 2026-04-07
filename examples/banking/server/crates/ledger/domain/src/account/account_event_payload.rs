use appletheia::event_payload;

use crate::currency_definition::CurrencyDefinitionId;

use super::{AccountBalance, AccountEventPayloadError, AccountId, AccountName, AccountOwner};

/// Represents the domain events emitted by an `Account` aggregate.
#[event_payload(error = AccountEventPayloadError)]
pub enum AccountEventPayload {
    Opened {
        id: AccountId,
        owner: AccountOwner,
        name: AccountName,
        currency_definition_id: CurrencyDefinitionId,
    },
    Renamed {
        name: AccountName,
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

    use crate::currency_definition::CurrencyDefinitionId;

    use super::{AccountBalance, AccountEventPayload, AccountId, AccountName, AccountOwner};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            AccountEventPayload::OPENED,
            appletheia::domain::EventName::new("opened")
        );
        assert_eq!(
            AccountEventPayload::RENAMED,
            appletheia::domain::EventName::new("renamed")
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
            owner: AccountOwner::User(banking_iam_domain::UserId::new()),
            name: AccountName::try_from("main").expect("account name should be valid"),
            currency_definition_id: CurrencyDefinitionId::new(),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("opened"));
        assert_eq!(value["data"]["owner"]["type"], serde_json::json!("user"));
    }

    #[test]
    fn serializes_renamed_payload_to_json() {
        let payload = AccountEventPayload::Renamed {
            name: AccountName::try_from("savings").expect("account name should be valid"),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("renamed"));
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
