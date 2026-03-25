use appletheia::event_payload;

use crate::core::Email;

use super::{
    UserDisplayName, UserEventPayloadError, UserId, UserIdentity, UserIdentityProvider,
    UserIdentitySubject, Username,
};

/// Represents the domain events emitted by a `User` aggregate.
#[event_payload(error = UserEventPayloadError)]
pub enum UserEventPayload {
    Registered {
        id: UserId,
        identity: UserIdentity,
    },
    Activated,
    Inactivated,
    Removed,
    ProfileReadied {
        username: Username,
        display_name: UserDisplayName,
    },
    UsernameChanged {
        username: Username,
    },
    DisplayNameChanged {
        display_name: UserDisplayName,
    },
    IdentityLinked {
        identity: UserIdentity,
    },
    IdentityEmailChanged {
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use crate::core::Email;

    use super::{
        UserDisplayName, UserEventPayload, UserId, UserIdentity, UserIdentityProvider,
        UserIdentitySubject, Username,
    };

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            UserEventPayload::REGISTERED,
            appletheia::domain::EventName::new("registered")
        );
        assert_eq!(
            UserEventPayload::PROFILE_READIED,
            appletheia::domain::EventName::new("profile_readied")
        );
        assert_eq!(
            UserEventPayload::ACTIVATED,
            appletheia::domain::EventName::new("activated")
        );
        assert_eq!(
            UserEventPayload::INACTIVATED,
            appletheia::domain::EventName::new("inactivated")
        );
        assert_eq!(
            UserEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
        );
        assert_eq!(
            UserEventPayload::USERNAME_CHANGED,
            appletheia::domain::EventName::new("username_changed")
        );
        assert_eq!(
            UserEventPayload::DISPLAY_NAME_CHANGED,
            appletheia::domain::EventName::new("display_name_changed")
        );
        assert_eq!(
            UserEventPayload::IDENTITY_LINKED,
            appletheia::domain::EventName::new("identity_linked")
        );
        assert_eq!(
            UserEventPayload::IDENTITY_EMAIL_CHANGED,
            appletheia::domain::EventName::new("identity_email_changed")
        );
    }

    #[test]
    fn payload_name_matches_variant() {
        let payload = UserEventPayload::ProfileReadied {
            username: Username::try_from("alice_example").expect("username should be valid"),
            display_name: UserDisplayName::try_from("Alice Example")
                .expect("display name should be valid"),
        };

        assert_eq!(payload.name(), UserEventPayload::PROFILE_READIED);
    }

    #[test]
    fn username_changed_payload_name_matches_variant() {
        let payload = UserEventPayload::UsernameChanged {
            username: Username::try_from("alice_example").expect("username should be valid"),
        };

        assert_eq!(payload.name(), UserEventPayload::USERNAME_CHANGED);
    }

    #[test]
    fn display_name_changed_payload_name_matches_variant() {
        let payload = UserEventPayload::DisplayNameChanged {
            display_name: UserDisplayName::try_from("Alice Example")
                .expect("display name should be valid"),
        };

        assert_eq!(payload.name(), UserEventPayload::DISPLAY_NAME_CHANGED);
    }

    #[test]
    fn identity_linked_payload_name_matches_variant() {
        let payload = UserEventPayload::IdentityLinked {
            identity: UserIdentity::new(
                UserIdentityProvider::try_from("https://accounts.example.com")
                    .expect("provider should be valid"),
                UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
                None,
            ),
        };

        assert_eq!(payload.name(), UserEventPayload::IDENTITY_LINKED);
    }

    #[test]
    fn identity_email_changed_payload_name_matches_variant() {
        let payload = UserEventPayload::IdentityEmailChanged {
            provider: UserIdentityProvider::try_from("https://accounts.example.com")
                .expect("provider should be valid"),
            subject: UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
            email: Some(Email::try_from("alice@example.com").expect("email should be valid")),
        };

        assert_eq!(payload.name(), UserEventPayload::IDENTITY_EMAIL_CHANGED);
    }

    #[test]
    fn serializes_payload_to_json() {
        let payload = UserEventPayload::Registered {
            id: UserId::new(),
            identity: UserIdentity::new(
                UserIdentityProvider::try_from("https://accounts.example.com")
                    .expect("provider should be valid"),
                UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
                Some(Email::try_from("alice@example.com").expect("email should be valid")),
            ),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("registered"));
    }
}
