use appletheia::event_payload;

use crate::{OrganizationId, UserId};

use super::{OrganizationMembershipEventPayloadError, OrganizationRoles};

/// Represents the domain events emitted by an `OrganizationMembership` aggregate.
#[event_payload(error = OrganizationMembershipEventPayloadError)]
pub enum OrganizationMembershipEventPayload {
    Created {
        organization_id: OrganizationId,
        user_id: UserId,
        roles: OrganizationRoles,
    },
    RolesChanged {
        organization_id: OrganizationId,
        user_id: UserId,
        roles: OrganizationRoles,
    },
    Removed {
        organization_id: OrganizationId,
        user_id: UserId,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use super::{OrganizationMembershipEventPayload, OrganizationRoles};
    use crate::{OrganizationId, UserId};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            OrganizationMembershipEventPayload::CREATED,
            appletheia::domain::EventName::new("created")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::ROLES_CHANGED,
            appletheia::domain::EventName::new("roles_changed")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
        );
    }

    #[test]
    fn created_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::Created {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationRoles::default(),
        };

        assert_eq!(payload.name(), OrganizationMembershipEventPayload::CREATED);
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
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationRoles::default(),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("created"));
    }
}
