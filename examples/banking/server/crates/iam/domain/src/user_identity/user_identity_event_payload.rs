use appletheia::event_payload;

use crate::core::Email;
use crate::user::UserId;

use super::{
    UserIdentityEventPayloadError, UserIdentityId, UserIdentityProvider, UserIdentitySubject,
};

/// Represents the domain events emitted by a `UserIdentity` aggregate.
#[event_payload(error = UserIdentityEventPayloadError)]
pub enum UserIdentityEventPayload {
    Created {
        id: UserIdentityId,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    },
    LinkedToUser {
        user_id: UserId,
    },
    EmailChanged {
        email: Option<Email>,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use super::{
        UserIdentityEventPayload, UserIdentityId, UserIdentityProvider, UserIdentitySubject,
    };
    use crate::core::Email;

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            UserIdentityEventPayload::CREATED,
            appletheia::domain::EventName::new("created")
        );
        assert_eq!(
            UserIdentityEventPayload::LINKED_TO_USER,
            appletheia::domain::EventName::new("linked_to_user")
        );
        assert_eq!(
            UserIdentityEventPayload::EMAIL_CHANGED,
            appletheia::domain::EventName::new("email_changed")
        );
    }

    #[test]
    fn payload_name_matches_variant() {
        let payload = UserIdentityEventPayload::EmailChanged { email: None };

        assert_eq!(payload.name(), UserIdentityEventPayload::EMAIL_CHANGED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let provider = UserIdentityProvider::try_from("https://accounts.example.com")
            .expect("provider should be valid");
        let subject = UserIdentitySubject::try_from("user-123").expect("subject should be valid");
        let payload = UserIdentityEventPayload::Created {
            id: UserIdentityId::new(&provider, &subject),
            provider,
            subject,
            email: Some(Email::try_from("alice@example.com").expect("email should be valid")),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("created"));
    }
}
