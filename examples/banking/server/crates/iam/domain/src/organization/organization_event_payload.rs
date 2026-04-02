use appletheia::event_payload;

use super::{OrganizationEventPayloadError, OrganizationId, OrganizationName};

/// Represents the domain events emitted by an `Organization` aggregate.
#[event_payload(error = OrganizationEventPayloadError)]
pub enum OrganizationEventPayload {
    Created {
        id: OrganizationId,
        name: OrganizationName,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use super::{OrganizationEventPayload, OrganizationId, OrganizationName};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            OrganizationEventPayload::CREATED,
            appletheia::domain::EventName::new("created")
        );
    }

    #[test]
    fn payload_name_matches_variant() {
        let payload = OrganizationEventPayload::Created {
            id: OrganizationId::new(),
            name: OrganizationName::try_from("Acme Labs").expect("name should be valid"),
        };

        assert_eq!(payload.name(), OrganizationEventPayload::CREATED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = OrganizationEventPayload::Created {
            id: OrganizationId::new(),
            name: OrganizationName::try_from("Acme Labs").expect("name should be valid"),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("created"));
    }
}
