use appletheia::event_payload;

use crate::{OrganizationId, UserId};

use super::OrganizationMembershipEventPayloadError;

/// Represents the domain events emitted by an `OrganizationMembership` aggregate.
#[event_payload(error = OrganizationMembershipEventPayloadError)]
pub enum OrganizationMembershipEventPayload {
    Created {
        organization_id: OrganizationId,
        user_id: UserId,
    },
    Deleted {
        organization_id: OrganizationId,
        user_id: UserId,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use super::OrganizationMembershipEventPayload;
    use crate::{OrganizationId, UserId};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            OrganizationMembershipEventPayload::CREATED,
            appletheia::domain::EventName::new("created")
        );
        assert_eq!(
            OrganizationMembershipEventPayload::DELETED,
            appletheia::domain::EventName::new("deleted")
        );
    }

    #[test]
    fn created_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::Created {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
        };

        assert_eq!(payload.name(), OrganizationMembershipEventPayload::CREATED);
    }

    #[test]
    fn deleted_payload_name_matches_variant() {
        let payload = OrganizationMembershipEventPayload::Deleted {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
        };

        assert_eq!(payload.name(), OrganizationMembershipEventPayload::DELETED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = OrganizationMembershipEventPayload::Created {
            organization_id: OrganizationId::new(),
            user_id: UserId::new(),
        };

        let value = payload
            .try_into_json_value()
            .expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("created"));
    }
}
