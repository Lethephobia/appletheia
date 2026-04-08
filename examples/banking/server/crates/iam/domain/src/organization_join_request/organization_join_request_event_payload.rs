use appletheia::event_payload;

use crate::{OrganizationId, UserId};

use super::{OrganizationJoinRequestEventPayloadError, OrganizationJoinRequestId};

/// Represents the domain events emitted by an `OrganizationJoinRequest` aggregate.
#[event_payload(error = OrganizationJoinRequestEventPayloadError)]
pub enum OrganizationJoinRequestEventPayload {
    Requested {
        id: OrganizationJoinRequestId,
        organization_id: OrganizationId,
        requester_id: UserId,
    },
    Approved {
        organization_id: OrganizationId,
        requester_id: UserId,
    },
    Rejected {
        organization_id: OrganizationId,
        requester_id: UserId,
    },
    Canceled {
        organization_id: OrganizationId,
        requester_id: UserId,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use super::{OrganizationJoinRequestEventPayload, OrganizationJoinRequestId};
    use crate::{OrganizationId, UserId};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            OrganizationJoinRequestEventPayload::REQUESTED,
            appletheia::domain::EventName::new("requested")
        );
        assert_eq!(
            OrganizationJoinRequestEventPayload::APPROVED,
            appletheia::domain::EventName::new("approved")
        );
        assert_eq!(
            OrganizationJoinRequestEventPayload::REJECTED,
            appletheia::domain::EventName::new("rejected")
        );
        assert_eq!(
            OrganizationJoinRequestEventPayload::CANCELED,
            appletheia::domain::EventName::new("canceled")
        );
    }

    #[test]
    fn requested_payload_name_matches_variant() {
        let payload = OrganizationJoinRequestEventPayload::Requested {
            id: OrganizationJoinRequestId::new(),
            organization_id: OrganizationId::new(),
            requester_id: UserId::new(),
        };

        assert_eq!(
            payload.name(),
            OrganizationJoinRequestEventPayload::REQUESTED
        );
    }

    #[test]
    fn approved_payload_name_matches_variant() {
        let payload = OrganizationJoinRequestEventPayload::Approved {
            organization_id: OrganizationId::new(),
            requester_id: UserId::new(),
        };

        assert_eq!(
            payload.name(),
            OrganizationJoinRequestEventPayload::APPROVED
        );
    }

    #[test]
    fn rejected_payload_name_matches_variant() {
        let payload = OrganizationJoinRequestEventPayload::Rejected {
            organization_id: OrganizationId::new(),
            requester_id: UserId::new(),
        };

        assert_eq!(
            payload.name(),
            OrganizationJoinRequestEventPayload::REJECTED
        );
    }

    #[test]
    fn canceled_payload_name_matches_variant() {
        let payload = OrganizationJoinRequestEventPayload::Canceled {
            organization_id: OrganizationId::new(),
            requester_id: UserId::new(),
        };

        assert_eq!(
            payload.name(),
            OrganizationJoinRequestEventPayload::CANCELED
        );
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = OrganizationJoinRequestEventPayload::Requested {
            id: OrganizationJoinRequestId::new(),
            organization_id: OrganizationId::new(),
            requester_id: UserId::new(),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("requested"));
    }
}
