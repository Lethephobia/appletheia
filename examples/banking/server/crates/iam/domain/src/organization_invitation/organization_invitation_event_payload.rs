use appletheia::event_payload;

use crate::{OrganizationId, OrganizationRoles, UserId};

use super::{
    OrganizationInvitationEventPayloadError, OrganizationInvitationExpiresAt,
    OrganizationInvitationIssuer,
};

/// Represents the domain events emitted by an `OrganizationInvitation` aggregate.
#[event_payload(error = OrganizationInvitationEventPayloadError)]
pub enum OrganizationInvitationEventPayload {
    Issued {
        organization_id: OrganizationId,
        invitee_id: UserId,
        roles: OrganizationRoles,
        issuer: OrganizationInvitationIssuer,
        expires_at: OrganizationInvitationExpiresAt,
    },
    Accepted {
        organization_id: OrganizationId,
        invitee_id: UserId,
        roles: OrganizationRoles,
    },
    Declined {
        organization_id: OrganizationId,
        invitee_id: UserId,
    },
    Canceled {
        organization_id: OrganizationId,
        invitee_id: UserId,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;
    use chrono::{Duration, Utc};

    use super::{OrganizationInvitationEventPayload, OrganizationInvitationIssuer};
    use crate::{OrganizationId, OrganizationRoles, UserId};

    fn expires_at() -> super::OrganizationInvitationExpiresAt {
        super::OrganizationInvitationExpiresAt::from(Utc::now() + Duration::minutes(10))
    }

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            OrganizationInvitationEventPayload::ISSUED,
            appletheia::domain::EventName::new("issued")
        );
        assert_eq!(
            OrganizationInvitationEventPayload::ACCEPTED,
            appletheia::domain::EventName::new("accepted")
        );
        assert_eq!(
            OrganizationInvitationEventPayload::DECLINED,
            appletheia::domain::EventName::new("declined")
        );
        assert_eq!(
            OrganizationInvitationEventPayload::CANCELED,
            appletheia::domain::EventName::new("canceled")
        );
    }

    #[test]
    fn issued_payload_name_matches_variant() {
        let payload = OrganizationInvitationEventPayload::Issued {
            organization_id: OrganizationId::new(),
            invitee_id: UserId::new(),
            roles: OrganizationRoles::default(),
            issuer: OrganizationInvitationIssuer::User(UserId::new()),
            expires_at: expires_at(),
        };

        assert_eq!(payload.name(), OrganizationInvitationEventPayload::ISSUED);
    }

    #[test]
    fn accepted_payload_name_matches_variant() {
        let payload = OrganizationInvitationEventPayload::Accepted {
            organization_id: OrganizationId::new(),
            invitee_id: UserId::new(),
            roles: OrganizationRoles::default(),
        };

        assert_eq!(payload.name(), OrganizationInvitationEventPayload::ACCEPTED);
    }

    #[test]
    fn declined_payload_name_matches_variant() {
        let payload = OrganizationInvitationEventPayload::Declined {
            organization_id: OrganizationId::new(),
            invitee_id: UserId::new(),
        };

        assert_eq!(payload.name(), OrganizationInvitationEventPayload::DECLINED);
    }

    #[test]
    fn canceled_payload_name_matches_variant() {
        let payload = OrganizationInvitationEventPayload::Canceled {
            organization_id: OrganizationId::new(),
            invitee_id: UserId::new(),
        };

        assert_eq!(payload.name(), OrganizationInvitationEventPayload::CANCELED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = OrganizationInvitationEventPayload::Issued {
            organization_id: OrganizationId::new(),
            invitee_id: UserId::new(),
            roles: OrganizationRoles::default(),
            issuer: OrganizationInvitationIssuer::User(UserId::new()),
            expires_at: expires_at(),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("issued"));
    }
}
