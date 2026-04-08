use appletheia::event_payload;

use crate::{OrganizationId, UserId};

use super::{OrganizationMembershipEventPayloadError, OrganizationMembershipId};

/// Represents the domain events emitted by an `OrganizationMembership` aggregate.
#[event_payload(error = OrganizationMembershipEventPayloadError)]
pub enum OrganizationMembershipEventPayload {
    Created {
        id: OrganizationMembershipId,
        organization_id: OrganizationId,
        user_id: UserId,
    },
    Activated {
        organization_id: OrganizationId,
        user_id: UserId,
    },
    Inactivated {
        organization_id: OrganizationId,
        user_id: UserId,
    },
    Removed {
        organization_id: OrganizationId,
        user_id: UserId,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use super::{OrganizationMembershipEventPayload, OrganizationMembershipId};
    use crate::{OrganizationId, UserId};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            OrganizationMembershipEventPayload::CREATED,
            appletheia::domain::EventName::new("created")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::ACTIVATED,
            appletheia::domain::EventName::new("activated")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::INACTIVATED,
            appletheia::domain::EventName::new("inactivated")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
        );
    }

    #[test]
    fn created_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::Created {
            id: OrganizationMembershipId::new(),
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
        };

        assert_eq!(payload.name(), OrganizationMembershipEventPayload::CREATED);
    }

    #[test]
    fn activated_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::Activated {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
        };

        assert_eq!(
            payload.name(),
            OrganizationMembershipEventPayload::ACTIVATED
        );
    }

    #[test]
    fn inactivated_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::Inactivated {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
        };

        assert_eq!(
            payload.name(),
            OrganizationMembershipEventPayload::INACTIVATED
        );
    }

    #[test]
    fn removed_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::Removed {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
        };

        assert_eq!(payload.name(), OrganizationMembershipEventPayload::REMOVED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = OrganizationMembershipEventPayload::Created {
            id: OrganizationMembershipId::new(),
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("created"));
    }
}
