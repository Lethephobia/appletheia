use appletheia::event_payload;

use crate::{OrganizationId, UserId};

use super::{
    OrganizationMembershipActivateRejectionReason, OrganizationMembershipDeactivateRejectionReason,
    OrganizationMembershipEventPayloadError, OrganizationMembershipId,
    OrganizationMembershipRemoveRejectionReason, OrganizationMembershipRoles,
    OrganizationMembershipRolesChangeRejectionReason,
};

/// Represents the domain events emitted by an `OrganizationMembership` aggregate.
#[event_payload(error = OrganizationMembershipEventPayloadError)]
pub enum OrganizationMembershipEventPayload {
    Created {
        id: OrganizationMembershipId,
        organization_id: OrganizationId,
        user_id: UserId,
        roles: OrganizationMembershipRoles,
    },
    RolesChanged {
        organization_id: OrganizationId,
        user_id: UserId,
        roles: OrganizationMembershipRoles,
    },
    RolesChangeRejected {
        organization_id: OrganizationId,
        user_id: UserId,
        roles: OrganizationMembershipRoles,
        reason: OrganizationMembershipRolesChangeRejectionReason,
    },
    Activated {
        organization_id: OrganizationId,
        user_id: UserId,
        roles: OrganizationMembershipRoles,
    },
    ActivateRejected {
        organization_id: OrganizationId,
        user_id: UserId,
        reason: OrganizationMembershipActivateRejectionReason,
    },
    Deactivated {
        organization_id: OrganizationId,
        user_id: UserId,
    },
    DeactivateRejected {
        organization_id: OrganizationId,
        user_id: UserId,
        reason: OrganizationMembershipDeactivateRejectionReason,
    },
    Removed {
        organization_id: OrganizationId,
        user_id: UserId,
    },
    RemoveRejected {
        organization_id: OrganizationId,
        user_id: UserId,
        reason: OrganizationMembershipRemoveRejectionReason,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use super::{
        OrganizationMembershipEventPayload, OrganizationMembershipId, OrganizationMembershipRoles,
    };
    use crate::{OrganizationId, OrganizationRole, UserId};

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
            OrganizationMembershipEventPayload::ROLES_CHANGE_REJECTED,
            appletheia::domain::EventName::new("roles_change_rejected")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::ACTIVATED,
            appletheia::domain::EventName::new("activated")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::ACTIVATE_REJECTED,
            appletheia::domain::EventName::new("activate_rejected")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::DEACTIVATED,
            appletheia::domain::EventName::new("deactivated")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::DEACTIVATE_REJECTED,
            appletheia::domain::EventName::new("deactivate_rejected")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::REMOVE_REJECTED,
            appletheia::domain::EventName::new("remove_rejected")
        );
    }

    #[test]
    fn created_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::Created {
            id: OrganizationMembershipId::new(),
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationMembershipRoles::new([OrganizationRole::Admin]),
        };

        assert_eq!(payload.name(), OrganizationMembershipEventPayload::CREATED);
    }

    #[test]
    fn activated_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::Activated {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationMembershipRoles::default(),
        };

        assert_eq!(
            payload.name(),
            OrganizationMembershipEventPayload::ACTIVATED
        );
    }

    #[test]
    fn deactivated_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::Deactivated {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
        };

        assert_eq!(
            payload.name(),
            OrganizationMembershipEventPayload::DEACTIVATED
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
    fn roles_changed_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::RolesChanged {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationMembershipRoles::new([OrganizationRole::Admin]),
        };

        assert_eq!(
            payload.name(),
            OrganizationMembershipEventPayload::ROLES_CHANGED
        );
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = OrganizationMembershipEventPayload::Created {
            id: OrganizationMembershipId::new(),
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationMembershipRoles::new([OrganizationRole::FinanceManager]),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("created"));
        assert_eq!(
            value["data"]["roles"],
            serde_json::json!([{ "type": "finance_manager" }])
        );
    }

    #[test]
    fn serializes_roles_changed_payload_to_json() {
        let payload = OrganizationMembershipEventPayload::RolesChanged {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationMembershipRoles::new([OrganizationRole::FinanceManager]),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("roles_changed"));
        assert_eq!(
            value["data"]["roles"],
            serde_json::json!([{ "type": "finance_manager" }])
        );
    }
}
