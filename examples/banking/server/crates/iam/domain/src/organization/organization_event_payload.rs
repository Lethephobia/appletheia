use appletheia::event_payload;

use super::OrganizationOwner;
use super::{OrganizationEventPayloadError, OrganizationHandle, OrganizationId, OrganizationName};

/// Represents the domain events emitted by an `Organization` aggregate.
#[event_payload(error = OrganizationEventPayloadError)]
pub enum OrganizationEventPayload {
    Created {
        id: OrganizationId,
        owner: OrganizationOwner,
        handle: OrganizationHandle,
        name: OrganizationName,
    },
    OwnershipTransferred {
        owner: OrganizationOwner,
    },
    HandleChanged {
        handle: OrganizationHandle,
    },
    NameChanged {
        name: OrganizationName,
    },
    Removed,
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use super::{
        OrganizationEventPayload, OrganizationHandle, OrganizationId, OrganizationName,
        OrganizationOwner,
    };

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            OrganizationEventPayload::CREATED,
            appletheia::domain::EventName::new("created")
        );
        assert_eq!(
            OrganizationEventPayload::OWNERSHIP_TRANSFERRED,
            appletheia::domain::EventName::new("ownership_transferred")
        );
        assert_eq!(
            OrganizationEventPayload::HANDLE_CHANGED,
            appletheia::domain::EventName::new("handle_changed")
        );
        assert_eq!(
            OrganizationEventPayload::NAME_CHANGED,
            appletheia::domain::EventName::new("name_changed")
        );
        assert_eq!(
            OrganizationEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
        );
    }

    #[test]
    fn payload_name_matches_variant() {
        let payload = OrganizationEventPayload::Created {
            id: OrganizationId::new(),
            owner: OrganizationOwner::User(crate::UserId::new()),
            handle: OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            name: OrganizationName::try_from("Acme Labs").expect("name should be valid"),
        };

        assert_eq!(payload.name(), OrganizationEventPayload::CREATED);
    }

    #[test]
    fn handle_changed_payload_name_matches_variant() {
        let payload = OrganizationEventPayload::HandleChanged {
            handle: OrganizationHandle::try_from("acme-labs-2").expect("handle should be valid"),
        };

        assert_eq!(payload.name(), OrganizationEventPayload::HANDLE_CHANGED);
    }

    #[test]
    fn ownership_transferred_payload_name_matches_variant() {
        let payload = OrganizationEventPayload::OwnershipTransferred {
            owner: OrganizationOwner::User(crate::UserId::new()),
        };

        assert_eq!(
            payload.name(),
            OrganizationEventPayload::OWNERSHIP_TRANSFERRED
        );
    }

    #[test]
    fn name_changed_payload_name_matches_variant() {
        let payload = OrganizationEventPayload::NameChanged {
            name: OrganizationName::try_from("Acme Labs 2").expect("name should be valid"),
        };

        assert_eq!(payload.name(), OrganizationEventPayload::NAME_CHANGED);
    }

    #[test]
    fn removed_payload_name_matches_variant() {
        let payload = OrganizationEventPayload::Removed;

        assert_eq!(payload.name(), OrganizationEventPayload::REMOVED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = OrganizationEventPayload::Created {
            id: OrganizationId::new(),
            owner: OrganizationOwner::User(crate::UserId::new()),
            handle: OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            name: OrganizationName::try_from("Acme Labs").expect("name should be valid"),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("created"));
        assert_eq!(value["data"]["owner"]["type"], serde_json::json!("user"));
    }

    #[test]
    fn serializes_ownership_transferred_payload_to_json() {
        let payload = OrganizationEventPayload::OwnershipTransferred {
            owner: OrganizationOwner::User(crate::UserId::new()),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("ownership_transferred"));
        assert_eq!(value["data"]["owner"]["type"], serde_json::json!("user"));
    }
}
