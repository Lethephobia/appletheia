use appletheia::event_payload;

use crate::core::CurrencyAmount;
use crate::currency::CurrencyId;

use super::{
    AccountCloseRejectionReason, AccountDepositRejectionReason, AccountEventPayloadError,
    AccountFreezeRejectionReason, AccountFundsReserveRejectionReason, AccountName,
    AccountNameChangeRejectionReason, AccountOwner, AccountOwnershipTransferRejectionReason,
    AccountReservedFundsCommitRejectionReason, AccountReservedFundsReleaseRejectionReason,
    AccountThawRejectionReason, AccountWithdrawRejectionReason,
};

/// Represents the domain events emitted by an `Account` aggregate.
#[event_payload(error = AccountEventPayloadError)]
pub enum AccountEventPayload {
    Opened {
        owner: AccountOwner,
        name: AccountName,
        currency_id: CurrencyId,
    },
    OwnershipTransferred {
        owner: AccountOwner,
    },
    OwnershipTransferRejected {
        owner: AccountOwner,
        reason: AccountOwnershipTransferRejectionReason,
    },
    NameChanged {
        name: AccountName,
    },
    NameChangeRejected {
        name: AccountName,
        reason: AccountNameChangeRejectionReason,
    },
    Deposited {
        amount: CurrencyAmount,
    },
    DepositRejected {
        amount: CurrencyAmount,
        reason: AccountDepositRejectionReason,
    },
    Withdrawn {
        amount: CurrencyAmount,
    },
    WithdrawRejected {
        amount: CurrencyAmount,
        reason: AccountWithdrawRejectionReason,
    },
    FundsReserved {
        amount: CurrencyAmount,
    },
    FundsReserveRejected {
        amount: CurrencyAmount,
        reason: AccountFundsReserveRejectionReason,
    },
    ReservedFundsReleased {
        amount: CurrencyAmount,
    },
    ReservedFundsReleaseRejected {
        amount: CurrencyAmount,
        reason: AccountReservedFundsReleaseRejectionReason,
    },
    ReservedFundsCommitted {
        amount: CurrencyAmount,
    },
    ReservedFundsCommitRejected {
        amount: CurrencyAmount,
        reason: AccountReservedFundsCommitRejectionReason,
    },
    Frozen,
    FreezeRejected {
        reason: AccountFreezeRejectionReason,
    },
    Thawed,
    ThawRejected {
        reason: AccountThawRejectionReason,
    },
    Closed,
    CloseRejected {
        reason: AccountCloseRejectionReason,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{EventName, EventPayload};
    use banking_iam_domain::{OrganizationId, UserId};

    use crate::currency::CurrencyId;

    use super::{AccountEventPayload, AccountName, AccountOwner, CurrencyAmount};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(AccountEventPayload::OPENED, EventName::new("opened"));
        assert_eq!(
            AccountEventPayload::OWNERSHIP_TRANSFERRED,
            EventName::new("ownership_transferred")
        );
        assert_eq!(
            AccountEventPayload::OWNERSHIP_TRANSFER_REJECTED,
            EventName::new("ownership_transfer_rejected")
        );
        assert_eq!(
            AccountEventPayload::NAME_CHANGED,
            EventName::new("name_changed")
        );
        assert_eq!(
            AccountEventPayload::NAME_CHANGE_REJECTED,
            EventName::new("name_change_rejected")
        );
        assert_eq!(AccountEventPayload::DEPOSITED, EventName::new("deposited"));
        assert_eq!(
            AccountEventPayload::DEPOSIT_REJECTED,
            EventName::new("deposit_rejected")
        );
        assert_eq!(AccountEventPayload::WITHDRAWN, EventName::new("withdrawn"));
        assert_eq!(
            AccountEventPayload::WITHDRAW_REJECTED,
            EventName::new("withdraw_rejected")
        );
        assert_eq!(
            AccountEventPayload::FUNDS_RESERVED,
            EventName::new("funds_reserved")
        );
        assert_eq!(
            AccountEventPayload::FUNDS_RESERVE_REJECTED,
            EventName::new("funds_reserve_rejected")
        );
        assert_eq!(
            AccountEventPayload::RESERVED_FUNDS_RELEASED,
            EventName::new("reserved_funds_released")
        );
        assert_eq!(
            AccountEventPayload::RESERVED_FUNDS_RELEASE_REJECTED,
            EventName::new("reserved_funds_release_rejected")
        );
        assert_eq!(
            AccountEventPayload::RESERVED_FUNDS_COMMITTED,
            EventName::new("reserved_funds_committed")
        );
        assert_eq!(
            AccountEventPayload::RESERVED_FUNDS_COMMIT_REJECTED,
            EventName::new("reserved_funds_commit_rejected")
        );
        assert_eq!(AccountEventPayload::FROZEN, EventName::new("frozen"));
        assert_eq!(
            AccountEventPayload::FREEZE_REJECTED,
            EventName::new("freeze_rejected")
        );
        assert_eq!(AccountEventPayload::THAWED, EventName::new("thawed"));
        assert_eq!(
            AccountEventPayload::THAW_REJECTED,
            EventName::new("thaw_rejected")
        );
        assert_eq!(AccountEventPayload::CLOSED, EventName::new("closed"));
        assert_eq!(
            AccountEventPayload::CLOSE_REJECTED,
            EventName::new("close_rejected")
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
            owner: AccountOwner::User(UserId::new()),
            name: AccountName::try_from("main").expect("account name should be valid"),
            currency_id: CurrencyId::new(),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("opened"));
        assert_eq!(value["data"]["owner"]["type"], serde_json::json!("user"));
    }

    #[test]
    fn serializes_organization_owned_payload_to_json() {
        let payload = AccountEventPayload::Opened {
            owner: AccountOwner::Organization(OrganizationId::new()),
            name: AccountName::try_from("ops").expect("account name should be valid"),
            currency_id: CurrencyId::new(),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(
            value["data"]["owner"]["type"],
            serde_json::json!("organization")
        );
    }

    #[test]
    fn serializes_ownership_transferred_payload_to_json() {
        let payload = AccountEventPayload::OwnershipTransferred {
            owner: AccountOwner::User(UserId::new()),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("ownership_transferred"));
        assert_eq!(value["data"]["owner"]["type"], serde_json::json!("user"));
    }

    #[test]
    fn serializes_name_changed_payload_to_json() {
        let payload = AccountEventPayload::NameChanged {
            name: AccountName::try_from("savings").expect("account name should be valid"),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("name_changed"));
    }

    #[test]
    fn serializes_balance_movement_payload_to_json() {
        let payload = AccountEventPayload::Deposited {
            amount: CurrencyAmount::new(10),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("deposited"));
    }

    #[test]
    fn serializes_reserved_funds_payload_to_json() {
        let payload = AccountEventPayload::FundsReserved {
            amount: CurrencyAmount::new(10),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("funds_reserved"));
    }
}
