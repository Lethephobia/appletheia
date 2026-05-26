use appletheia::event_payload;

use super::{
    PayoutDestinationEventPayloadError, PayoutDestinationId, PayoutDestinationOwner,
    PayoutDestinationRemoveRejectionReason, PayoutDestinationTokenAccountOwnerAddress,
};

/// Represents the domain events emitted by a `PayoutDestination` aggregate.
#[event_payload(error = PayoutDestinationEventPayloadError)]
pub enum PayoutDestinationEventPayload {
    Registered {
        id: PayoutDestinationId,
        owner: PayoutDestinationOwner,
        token_account_owner_address: PayoutDestinationTokenAccountOwnerAddress,
    },
    Removed,
    RemoveRejected {
        reason: PayoutDestinationRemoveRejectionReason,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;
    use banking_iam_domain::UserId;

    use super::{
        PayoutDestinationEventPayload, PayoutDestinationId, PayoutDestinationOwner,
        PayoutDestinationTokenAccountOwnerAddress,
    };

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            PayoutDestinationEventPayload::REGISTERED,
            appletheia::domain::EventName::new("registered")
        );
        assert_eq!(
            PayoutDestinationEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
        );
        assert_eq!(
            PayoutDestinationEventPayload::REMOVE_REJECTED,
            appletheia::domain::EventName::new("remove_rejected")
        );
    }

    #[test]
    fn payload_name_matches_variant() {
        let payload = PayoutDestinationEventPayload::Removed;

        assert_eq!(payload.name(), PayoutDestinationEventPayload::REMOVED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = PayoutDestinationEventPayload::Registered {
            id: PayoutDestinationId::new(),
            owner: PayoutDestinationOwner::User(UserId::new()),
            token_account_owner_address: PayoutDestinationTokenAccountOwnerAddress::try_from(
                "11111111111111111111111111111111",
            )
            .expect("address should be valid"),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("registered"));
    }
}
