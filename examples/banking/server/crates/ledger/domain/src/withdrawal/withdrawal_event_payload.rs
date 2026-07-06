use appletheia::event_payload;

use crate::account::AccountId;
use crate::core::{CurrencyAmount, OnchainTransactionId, TokenAccountOwnerAddress};
use crate::currency::CurrencyId;

use super::{
    WithdrawalCompleteRejectionReason, WithdrawalEventPayloadError, WithdrawalFailRejectionReason,
    WithdrawalFailureReason, WithdrawalId, WithdrawalRequestRejectionReason,
    WithdrawalTokenTransferRejectionReason,
};

/// Represents the domain events emitted by a `Withdrawal` aggregate.
#[event_payload(error = WithdrawalEventPayloadError)]
pub enum WithdrawalEventPayload {
    Requested {
        id: WithdrawalId,
        account_id: AccountId,
        currency_id: CurrencyId,
        token_account_owner_address: TokenAccountOwnerAddress,
        amount: CurrencyAmount,
    },
    RequestRejected {
        id: WithdrawalId,
        account_id: AccountId,
        currency_id: CurrencyId,
        token_account_owner_address: TokenAccountOwnerAddress,
        amount: CurrencyAmount,
        reason: WithdrawalRequestRejectionReason,
    },
    TokenTransferred {
        onchain_transaction_id: OnchainTransactionId,
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

    use crate::account::AccountId;
    use crate::core::{CurrencyAmount, TokenAccountOwnerAddress};
    use crate::currency::CurrencyId;

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
            token_account_owner_address: TokenAccountOwnerAddress::try_from(
                "11111111111111111111111111111111",
            )
            .expect("token account owner address should be valid"),
            amount: CurrencyAmount::new(100),
        };

        let value = payload.into_json_value().expect("payload should serialize");
        assert_eq!(value["type"], serde_json::json!("requested"));
    }
}
