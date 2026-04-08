use appletheia::event_payload;

use crate::account::AccountId;
use crate::core::CurrencyAmount;

use super::{TransferEventPayloadError, TransferId};

/// Represents the domain events emitted by a `Transfer` aggregate.
#[event_payload(error = TransferEventPayloadError)]
pub enum TransferEventPayload {
    Requested {
        id: TransferId,
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: CurrencyAmount,
    },
    Completed,
    Failed,
    Cancelled,
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
            TransferEventPayload::COMPLETED,
            appletheia::domain::EventName::new("completed")
        );
        assert_eq!(
            TransferEventPayload::FAILED,
            appletheia::domain::EventName::new("failed")
        );
        assert_eq!(
            TransferEventPayload::CANCELLED,
            appletheia::domain::EventName::new("cancelled")
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
