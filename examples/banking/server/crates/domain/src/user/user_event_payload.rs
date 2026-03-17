use appletheia::event_payload;

use crate::core::{Email, Username};

use super::{UserEventPayloadError, UserId};

/// Represents the domain events emitted by a `User` aggregate.
#[event_payload(error = UserEventPayloadError)]
pub enum UserEventPayload {
    Registered {
        id: UserId,
        email: Email,
        username: Username,
    },
    EmailChanged {
        email: Email,
    },
    UsernameChanged {
        username: Username,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use crate::core::{Email, Username};

    use super::{UserEventPayload, UserId};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            UserEventPayload::REGISTERED,
            appletheia::domain::EventName::new("registered")
        );
        assert_eq!(
            UserEventPayload::EMAIL_CHANGED,
            appletheia::domain::EventName::new("email_changed")
        );
        assert_eq!(
            UserEventPayload::USERNAME_CHANGED,
            appletheia::domain::EventName::new("username_changed")
        );
    }

    #[test]
    fn payload_name_matches_variant() {
        let payload = UserEventPayload::UsernameChanged {
            username: Username::try_from("Alice Example").expect("username should be valid"),
        };

        assert_eq!(payload.name(), UserEventPayload::USERNAME_CHANGED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = UserEventPayload::Registered {
            id: UserId::new(),
            email: Email::try_from("alice@example.com").expect("email should be valid"),
            username: Username::try_from("Alice").expect("username should be valid"),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("registered"));
    }
}
