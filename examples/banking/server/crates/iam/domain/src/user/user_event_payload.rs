use appletheia::event_payload;

use super::{UserDisplayName, UserEventPayloadError, UserId, Username};

/// Represents the domain events emitted by a `User` aggregate.
#[event_payload(error = UserEventPayloadError)]
pub enum UserEventPayload {
    Registered {
        id: UserId,
        username: Username,
    },
    UsernameChanged {
        username: Username,
    },
    DisplayNameChanged {
        display_name: Option<UserDisplayName>,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use super::{UserDisplayName, UserEventPayload, UserId, Username};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            UserEventPayload::REGISTERED,
            appletheia::domain::EventName::new("registered")
        );
        assert_eq!(
            UserEventPayload::USERNAME_CHANGED,
            appletheia::domain::EventName::new("username_changed")
        );
        assert_eq!(
            UserEventPayload::DISPLAY_NAME_CHANGED,
            appletheia::domain::EventName::new("display_name_changed")
        );
    }

    #[test]
    fn payload_name_matches_variant() {
        let payload = UserEventPayload::UsernameChanged {
            username: Username::try_from("alice_example").expect("username should be valid"),
        };

        assert_eq!(payload.name(), UserEventPayload::USERNAME_CHANGED);
    }

    #[test]
    fn display_name_payload_name_matches_variant() {
        let payload = UserEventPayload::DisplayNameChanged {
            display_name: Some(
                UserDisplayName::try_from("Alice Example").expect("display name should be valid"),
            ),
        };

        assert_eq!(payload.name(), UserEventPayload::DISPLAY_NAME_CHANGED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = UserEventPayload::Registered {
            id: UserId::new(),
            username: Username::try_from("alice").expect("username should be valid"),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("registered"));
    }
}
