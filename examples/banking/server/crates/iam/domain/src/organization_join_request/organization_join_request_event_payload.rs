use appletheia::event_payload;

use crate::{OrganizationId, UserId};

use super::{
    OrganizationJoinRequestApproveRejectionReason, OrganizationJoinRequestCancelRejectionReason,
    OrganizationJoinRequestEventPayloadError, OrganizationJoinRequestId,
    OrganizationJoinRequestRejectRejectionReason, OrganizationJoinRequestStatus,
};

/// Represents the domain events emitted by an `OrganizationJoinRequest` aggregate.
#[event_payload(error = OrganizationJoinRequestEventPayloadError)]
pub enum OrganizationJoinRequestEventPayload {
    Requested {
        id: OrganizationJoinRequestId,
        organization_id: OrganizationId,
        requester_id: UserId,
        status: OrganizationJoinRequestStatus,
    },
    Approved {
        organization_id: OrganizationId,
        requester_id: UserId,
    },
    ApproveRejected {
        organization_id: OrganizationId,
        requester_id: UserId,
        reason: OrganizationJoinRequestApproveRejectionReason,
    },
    Rejected {
        organization_id: OrganizationId,
        requester_id: UserId,
    },
    RejectRejected {
        organization_id: OrganizationId,
        requester_id: UserId,
        reason: OrganizationJoinRequestRejectRejectionReason,
    },
    Canceled {
        organization_id: OrganizationId,
        requester_id: UserId,
    },
    CancelRejected {
        organization_id: OrganizationId,
        requester_id: UserId,
        reason: OrganizationJoinRequestCancelRejectionReason,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use super::{
        OrganizationJoinRequestEventPayload, OrganizationJoinRequestId,
        OrganizationJoinRequestStatus,
    };
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
            OrganizationJoinRequestEventPayload::APPROVE_REJECTED,
            appletheia::domain::EventName::new("approve_rejected")
        );
        assert_eq!(
            OrganizationJoinRequestEventPayload::REJECTED,
            appletheia::domain::EventName::new("rejected")
        );
        assert_eq!(
            OrganizationJoinRequestEventPayload::REJECT_REJECTED,
            appletheia::domain::EventName::new("reject_rejected")
        );
        assert_eq!(
            OrganizationJoinRequestEventPayload::CANCELED,
            appletheia::domain::EventName::new("canceled")
        );
        assert_eq!(
            OrganizationJoinRequestEventPayload::CANCEL_REJECTED,
            appletheia::domain::EventName::new("cancel_rejected")
        );
    }

    #[test]
    fn requested_payload_name_matches_variant() {
        let payload = OrganizationJoinRequestEventPayload::Requested {
            id: OrganizationJoinRequestId::new(),
            organization_id: OrganizationId::new(),
            requester_id: UserId::new(),
            status: OrganizationJoinRequestStatus::Pending,
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
            status: OrganizationJoinRequestStatus::Pending,
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("requested"));
    }
}
