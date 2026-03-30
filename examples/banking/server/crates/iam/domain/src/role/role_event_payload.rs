use appletheia::event_payload;

use super::{RoleEventPayloadError, RoleId, RoleName};

/// Represents the domain events emitted by a `Role` aggregate.
#[event_payload(error = RoleEventPayloadError)]
pub enum RoleEventPayload {
    Created { id: RoleId, name: RoleName },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use super::{RoleEventPayload, RoleId, RoleName};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            RoleEventPayload::CREATED,
            appletheia::domain::EventName::new("created")
        );
    }

    #[test]
    fn payload_name_matches_variant() {
        let payload = RoleEventPayload::Created {
            id: RoleId::admin(),
            name: RoleName::admin(),
        };

        assert_eq!(payload.name(), RoleEventPayload::CREATED);
    }
}
