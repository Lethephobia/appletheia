use appletheia::event_payload;

use crate::{OrganizationId, OrganizationRole, OrganizationRoles, UserId};

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
        roles: OrganizationRoles,
    },
    Inactivated {
        organization_id: OrganizationId,
        user_id: UserId,
    },
    Removed {
        organization_id: OrganizationId,
        user_id: UserId,
    },
    RoleGranted {
        organization_id: OrganizationId,
        user_id: UserId,
        role: OrganizationRole,
    },
    RoleRevoked {
        organization_id: OrganizationId,
        user_id: UserId,
        role: OrganizationRole,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use super::{OrganizationMembershipEventPayload, OrganizationMembershipId};
    use crate::{OrganizationId, OrganizationRole, OrganizationRoles, UserId};

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
        assert_eq!(
            OrganizationMembershipEventPayload::ROLE_GRANTED,
            appletheia::domain::EventName::new("role_granted")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::ROLE_REVOKED,
            appletheia::domain::EventName::new("role_revoked")
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
            roles: OrganizationRoles::default(),
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
    fn role_granted_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::RoleGranted {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            role: OrganizationRole::Admin,
        };

        assert_eq!(
            payload.name(),
            OrganizationMembershipEventPayload::ROLE_GRANTED
        );
    }

    #[test]
    fn role_revoked_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::RoleRevoked {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            role: OrganizationRole::Admin,
        };

        assert_eq!(
            payload.name(),
            OrganizationMembershipEventPayload::ROLE_REVOKED
        );
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

    #[test]
    fn serializes_role_granted_payload_to_json() {
        let payload = OrganizationMembershipEventPayload::RoleGranted {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            role: OrganizationRole::FinanceManager,
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("role_granted"));
        assert_eq!(
            value["data"]["role"],
            serde_json::json!({ "type": "finance_manager" })
        );
    }
}
