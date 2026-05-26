use appletheia::event_payload;

use crate::{
    account::AccountId, core::CurrencyAmount, currency::CurrencyId,
    payout_destination::PayoutDestinationId,
};

use super::{
    WithdrawalCompleteRejectionReason, WithdrawalEventPayloadError, WithdrawalFailRejectionReason,
    WithdrawalFailureReason, WithdrawalId, WithdrawalOnchainTransactionId,
    WithdrawalRequestRejectionReason, WithdrawalTokenTransferRejectionReason,
};

/// Represents the domain events emitted by a `Withdrawal` aggregate.
#[event_payload(error = WithdrawalEventPayloadError)]
pub enum WithdrawalEventPayload {
    Requested {
        id: WithdrawalId,
        account_id: AccountId,
        currency_id: CurrencyId,
        payout_destination_id: PayoutDestinationId,
        amount: CurrencyAmount,
    },
    RequestRejected {
        id: WithdrawalId,
        account_id: AccountId,
        currency_id: CurrencyId,
        payout_destination_id: PayoutDestinationId,
        amount: CurrencyAmount,
        reason: WithdrawalRequestRejectionReason,
    },
    TokenTransferred {
        onchain_transaction_id: WithdrawalOnchainTransactionId,
    },
    TokenTransferRejected {
        reason: WithdrawalTokenTransferRejectionReason,
    },
    Completed,
    CompleteRejected {
        reason: WithdrawalCompleteRejectionReason,
    },
    Failed {
        reason: WithdrawalFailureReason,
    },
    FailRejected {
        reason: WithdrawalFailRejectionReason,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use crate::{
        account::AccountId, core::CurrencyAmount, currency::CurrencyId,
        payout_destination::PayoutDestinationId,
    };

    use super::{WithdrawalEventPayload, WithdrawalId};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            WithdrawalEventPayload::REQUESTED,
            appletheia::domain::EventName::new("requested")
        );
        assert_eq!(
            WithdrawalEventPayload::REQUEST_REJECTED,
            appletheia::domain::EventName::new("request_rejected")
        );
        assert_eq!(
            WithdrawalEventPayload::TOKEN_TRANSFERRED,
            appletheia::domain::EventName::new("token_transferred")
        );
        assert_eq!(
            WithdrawalEventPayload::TOKEN_TRANSFER_REJECTED,
            appletheia::domain::EventName::new("token_transfer_rejected")
        );
        assert_eq!(
            WithdrawalEventPayload::COMPLETED,
            appletheia::domain::EventName::new("completed")
        );
        assert_eq!(
            WithdrawalEventPayload::COMPLETE_REJECTED,
            appletheia::domain::EventName::new("complete_rejected")
        );
        assert_eq!(
            WithdrawalEventPayload::FAILED,
            appletheia::domain::EventName::new("failed")
        );
        assert_eq!(
            WithdrawalEventPayload::FAIL_REJECTED,
            appletheia::domain::EventName::new("fail_rejected")
        );
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = WithdrawalEventPayload::Requested {
            id: WithdrawalId::new(),
            account_id: AccountId::new(),
            currency_id: CurrencyId::new(),
            payout_destination_id: PayoutDestinationId::new(),
            amount: CurrencyAmount::new(100),
        };

        let value = payload.into_json_value().expect("payload should serialize");
        assert_eq!(value["type"], serde_json::json!("requested"));
    }
}
