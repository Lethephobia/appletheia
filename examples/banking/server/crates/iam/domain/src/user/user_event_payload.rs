use appletheia::event_payload;

use crate::core::Email;

use super::{
    UserBio, UserDisplayName, UserEventPayloadError, UserId, UserIdentity, UserIdentityProvider,
    UserIdentitySubject, UserPictureRef, Username,
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
    UsernameChanged {
        username: Username,
    },
    DisplayNameChanged {
        display_name: UserDisplayName,
    },
    BioChanged {
        bio: Option<UserBio>,
    },
    PictureChanged {
        picture: Option<UserPictureRef>,
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
    use crate::{UserBio, UserDisplayName, UserPictureRef, UserPictureUrl};

    use super::{
        UserEventPayload, UserId, UserIdentity, UserIdentityProvider, UserIdentitySubject,
    };

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
        assert_eq!(
            UserEventPayload::BIO_CHANGED,
            appletheia::domain::EventName::new("bio_changed")
        );
        assert_eq!(
            UserEventPayload::PICTURE_CHANGED,
            appletheia::domain::EventName::new("picture_changed")
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
            UserEventPayload::IDENTITY_LINKED,
            appletheia::domain::EventName::new("identity_linked")
        );
        assert_eq!(
            UserEventPayload::IDENTITY_EMAIL_CHANGED,
            appletheia::domain::EventName::new("identity_email_changed")
        );
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
    fn serializes_bio_changed_payload_to_json() {
        let payload = UserEventPayload::BioChanged {
            bio: Some(UserBio::try_from("Banking enthusiast").expect("bio should be valid")),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("bio_changed"));
        assert_eq!(
            value["data"]["bio"],
            serde_json::json!("Banking enthusiast")
        );
    }

    #[test]
    fn serializes_picture_changed_payload_to_json() {
        let payload = UserEventPayload::PictureChanged {
            picture: Some(UserPictureRef::external_url(
                UserPictureUrl::try_from("https://cdn.example.com/alice.png")
                    .expect("picture URL should be valid"),
            )),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("picture_changed"));
        assert!(value["data"]["picture"].is_object());
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
    fn serializes_identity_linked_payload_to_json() {
        let payload = UserEventPayload::IdentityLinked {
            identity: UserIdentity::new(
                UserIdentityProvider::try_from("https://accounts.example.com")
                    .expect("provider should be valid"),
                UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
                Some(Email::try_from("alice@example.com").expect("email should be valid")),
            ),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("identity_linked"));
        assert_eq!(
            value["data"]["identity"]["provider"],
            serde_json::json!("https://accounts.example.com")
        );
    }

    #[test]
    fn serializes_registered_payload_to_json() {
        let payload = UserEventPayload::Registered {
            id: UserId::new(),
            identity: UserIdentity::new(
                UserIdentityProvider::try_from("https://accounts.example.com")
                    .expect("provider should be valid"),
                UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
                None,
            ),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("registered"));
        assert_eq!(
            value["data"]["identity"]["subject"],
            serde_json::json!("user-123")
        );
    }
}
