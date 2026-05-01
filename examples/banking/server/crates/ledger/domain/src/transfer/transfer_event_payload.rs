use appletheia::event_payload;

use crate::account::AccountId;
use crate::core::CurrencyAmount;

use super::{
    TransferCancelRejectionReason, TransferCompleteRejectionReason, TransferEventPayloadError,
    TransferFailRejectionReason, TransferFailureReason, TransferId, TransferRequestRejectionReason,
};

/// Represents the domain events emitted by a `Transfer` aggregate.
#[event_payload(error = TransferEventPayloadError)]
pub enum TransferEventPayload {
    Requested {
        id: TransferId,
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: CurrencyAmount,
    },
    RequestRejected {
        id: TransferId,
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: CurrencyAmount,
        reason: TransferRequestRejectionReason,
    },
    Completed,
    CompleteRejected {
        reason: TransferCompleteRejectionReason,
    },
    Failed {
        reason: TransferFailureReason,
    },
    FailRejected {
        reason: TransferFailRejectionReason,
    },
    Cancelled,
    CancelRejected {
        reason: TransferCancelRejectionReason,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use crate::account::AccountId;
    use crate::core::CurrencyAmount;

    use super::{TransferEventPayload, TransferId};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            TransferEventPayload::REQUESTED,
            appletheia::domain::EventName::new("requested")
        );
        assert_eq!(
            TransferEventPayload::REQUEST_REJECTED,
            appletheia::domain::EventName::new("request_rejected")
        );
        assert_eq!(
            TransferEventPayload::COMPLETED,
            appletheia::domain::EventName::new("completed")
        );
        assert_eq!(
            TransferEventPayload::COMPLETE_REJECTED,
            appletheia::domain::EventName::new("complete_rejected")
        );
        assert_eq!(
            TransferEventPayload::FAILED,
            appletheia::domain::EventName::new("failed")
        );
        assert_eq!(
            TransferEventPayload::FAIL_REJECTED,
            appletheia::domain::EventName::new("fail_rejected")
        );
        assert_eq!(
            TransferEventPayload::CANCELLED,
            appletheia::domain::EventName::new("cancelled")
        );
        assert_eq!(
            TransferEventPayload::CANCEL_REJECTED,
            appletheia::domain::EventName::new("cancel_rejected")
        );
    }

    #[test]
    fn payload_name_matches_variant() {
        let payload = TransferEventPayload::Completed;

        assert_eq!(payload.name(), TransferEventPayload::COMPLETED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = TransferEventPayload::Requested {
            id: TransferId::new(),
            from_account_id: AccountId::new(),
            to_account_id: AccountId::new(),
            amount: CurrencyAmount::new(100),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("requested"));
    }
}
