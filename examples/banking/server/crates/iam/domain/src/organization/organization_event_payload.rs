use appletheia::event_payload;

use super::{OrganizationEventPayloadError, OrganizationHandle, OrganizationId, OrganizationName};

/// Represents the domain events emitted by an `Organization` aggregate.
#[event_payload(error = OrganizationEventPayloadError)]
pub enum OrganizationEventPayload {
    Created {
        id: OrganizationId,
        handle: OrganizationHandle,
        name: OrganizationName,
    },
    HandleChanged {
        handle: OrganizationHandle,
    },
    Removed,
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use super::{OrganizationEventPayload, OrganizationHandle, OrganizationId, OrganizationName};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            OrganizationEventPayload::CREATED,
            appletheia::domain::EventName::new("created")
        );
        assert_eq!(
            OrganizationEventPayload::HANDLE_CHANGED,
            appletheia::domain::EventName::new("handle_changed")
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
    fn removed_payload_name_matches_variant() {
        let payload = OrganizationEventPayload::Removed;

        assert_eq!(payload.name(), OrganizationEventPayload::REMOVED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = OrganizationEventPayload::Created {
            id: OrganizationId::new(),
            handle: OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            name: OrganizationName::try_from("Acme Labs").expect("name should be valid"),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("created"));
    }
}
