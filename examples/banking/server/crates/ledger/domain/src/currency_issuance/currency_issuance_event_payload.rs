use appletheia::event_payload;

use crate::account::AccountId;
use crate::core::CurrencyAmount;
use crate::currency::CurrencyId;

use super::{
    CurrencyIssuanceCompleteRejectionReason, CurrencyIssuanceEventPayloadError,
    CurrencyIssuanceFailRejectionReason, CurrencyIssuanceFailureReason,
    CurrencyIssuanceIssueRejectionReason,
};

/// Represents the domain events emitted by a `CurrencyIssuance` aggregate.
#[event_payload(error = CurrencyIssuanceEventPayloadError)]
pub enum CurrencyIssuanceEventPayload {
    Issued {
        currency_id: CurrencyId,
        destination_account_id: AccountId,
        amount: CurrencyAmount,
    },
    IssueRejected {
        currency_id: CurrencyId,
        destination_account_id: AccountId,
        amount: CurrencyAmount,
        reason: CurrencyIssuanceIssueRejectionReason,
    },
    Completed,
    CompleteRejected {
        reason: CurrencyIssuanceCompleteRejectionReason,
    },
    Failed {
        reason: CurrencyIssuanceFailureReason,
    },
    FailRejected {
        reason: CurrencyIssuanceFailRejectionReason,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use crate::account::AccountId;
    use crate::core::CurrencyAmount;
    use crate::currency::CurrencyId;

    use super::CurrencyIssuanceEventPayload;

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            CurrencyIssuanceEventPayload::ISSUED,
            appletheia::domain::EventName::new("issued")
        );
        assert_eq!(
            CurrencyIssuanceEventPayload::ISSUE_REJECTED,
            appletheia::domain::EventName::new("issue_rejected")
        );
        assert_eq!(
            CurrencyIssuanceEventPayload::COMPLETED,
            appletheia::domain::EventName::new("completed")
        );
        assert_eq!(
            CurrencyIssuanceEventPayload::COMPLETE_REJECTED,
            appletheia::domain::EventName::new("complete_rejected")
        );
        assert_eq!(
            CurrencyIssuanceEventPayload::FAILED,
            appletheia::domain::EventName::new("failed")
        );
        assert_eq!(
            CurrencyIssuanceEventPayload::FAIL_REJECTED,
            appletheia::domain::EventName::new("fail_rejected")
        );
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = CurrencyIssuanceEventPayload::Issued {
            currency_id: CurrencyId::new(),
            destination_account_id: AccountId::new(),
            amount: CurrencyAmount::new(100),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("issued"));
    }
}
