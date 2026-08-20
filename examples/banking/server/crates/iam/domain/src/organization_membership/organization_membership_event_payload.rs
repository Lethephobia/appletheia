use appletheia::event_payload;

use crate::{OrganizationId, UserId};

use super::{
    OrganizationMembershipCreateRejectionReason, OrganizationMembershipEventPayloadError,
    OrganizationMembershipRemoveRejectionReason, OrganizationMembershipRolesChangeRejectionReason,
    OrganizationRoles,
};

/// Represents the domain events emitted by an `OrganizationMembership` aggregate.
#[event_payload(error = OrganizationMembershipEventPayloadError)]
pub enum OrganizationMembershipEventPayload {
    Created {
        organization_id: OrganizationId,
        user_id: UserId,
        roles: OrganizationRoles,
    },
    CreateRejected {
        organization_id: OrganizationId,
        user_id: UserId,
        roles: OrganizationRoles,
        reason: OrganizationMembershipCreateRejectionReason,
    },
    RolesChanged {
        organization_id: OrganizationId,
        user_id: UserId,
        roles: OrganizationRoles,
    },
    RolesChangeRejected {
        organization_id: OrganizationId,
        user_id: UserId,
        roles: OrganizationRoles,
        reason: OrganizationMembershipRolesChangeRejectionReason,
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
        OrganizationMembershipCreateRejectionReason, OrganizationMembershipEventPayload,
        OrganizationRoles,
    };
    use crate::{OrganizationId, UserId};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            OrganizationMembershipEventPayload::CREATED,
            appletheia::domain::EventName::new("created")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::CREATE_REJECTED,
            appletheia::domain::EventName::new("create_rejected")
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
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationRoles::default(),
        };

        assert_eq!(payload.name(), OrganizationMembershipEventPayload::CREATED);
    }

    #[test]
    fn create_rejected_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::CreateRejected {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationRoles::default(),
            reason: OrganizationMembershipCreateRejectionReason::AlreadyMember,
        };

        assert_eq!(
            payload.name(),
            OrganizationMembershipEventPayload::CREATE_REJECTED
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
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
            roles: OrganizationRoles::default(),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("created"));
    }
}
