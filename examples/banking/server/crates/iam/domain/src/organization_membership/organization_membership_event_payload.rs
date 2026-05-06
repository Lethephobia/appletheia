use appletheia::event_payload;

use crate::{OrganizationId, OrganizationRole, UserId};

use super::{
    OrganizationMembershipActivateRejectionReason, OrganizationMembershipDeactivateRejectionReason,
    OrganizationMembershipEventPayloadError, OrganizationMembershipId,
    OrganizationMembershipRemoveRejectionReason, OrganizationMembershipRoleGrantRejectionReason,
    OrganizationMembershipRoleRevokeRejectionReason,
};

/// Represents the domain events emitted by an `OrganizationMembership` aggregate.
#[event_payload(error = OrganizationMembershipEventPayloadError)]
pub enum OrganizationMembershipEventPayload {
    Created {
        id: OrganizationMembershipId,
        organization_id: OrganizationId,
        user_id: UserId,
    },
    RoleGranted {
        organization_id: OrganizationId,
        user_id: UserId,
        role: OrganizationRole,
    },
    RoleGrantRejected {
        organization_id: OrganizationId,
        user_id: UserId,
        role: OrganizationRole,
        reason: OrganizationMembershipRoleGrantRejectionReason,
    },
    RoleRevoked {
        organization_id: OrganizationId,
        user_id: UserId,
        role: OrganizationRole,
    },
    RoleRevokeRejected {
        organization_id: OrganizationId,
        user_id: UserId,
        role: OrganizationRole,
        reason: OrganizationMembershipRoleRevokeRejectionReason,
    },
    Activated {
        organization_id: OrganizationId,
        user_id: UserId,
        roles: Vec<OrganizationRole>,
    },
    ActivateRejected {
        organization_id: OrganizationId,
        user_id: UserId,
        reason: OrganizationMembershipActivateRejectionReason,
    },
    Inactivated {
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

    use super::{OrganizationMembershipEventPayload, OrganizationMembershipId};
    use crate::{OrganizationId, OrganizationRole, UserId};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            OrganizationMembershipEventPayload::CREATED,
            appletheia::domain::EventName::new("created")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::ROLE_GRANTED,
            appletheia::domain::EventName::new("role_granted")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::ROLE_GRANT_REJECTED,
            appletheia::domain::EventName::new("role_grant_rejected")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::ROLE_REVOKED,
            appletheia::domain::EventName::new("role_revoked")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::ROLE_REVOKE_REJECTED,
            appletheia::domain::EventName::new("role_revoke_rejected")
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
            OrganizationMembershipEventPayload::INACTIVATED,
            appletheia::domain::EventName::new("inactivated")
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
        };

        assert_eq!(payload.name(), OrganizationMembershipEventPayload::CREATED);
    }

    #[test]
    fn activated_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::Activated {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: Vec::new(),
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
