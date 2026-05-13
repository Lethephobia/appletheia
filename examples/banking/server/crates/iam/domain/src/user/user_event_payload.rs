use appletheia::event_payload;

use crate::core::Email;

use super::{
    UserBio, UserBioChangeRejectionReason, UserDisplayName, UserDisplayNameChangeRejectionReason,
    UserEventPayloadError, UserId, UserIdentity, UserIdentityEmailChangeRejectionReason,
    UserIdentityLinkRejectionReason, UserIdentityProvider, UserIdentitySubject,
    UserPictureChangeRejectionReason, UserPictureRef, UserStatus, UserStatusRejectionReason,
    UserUsernameChangeRejectionReason, Username,
};

/// Represents the domain events emitted by a `User` aggregate.
#[event_payload(error = UserEventPayloadError)]
pub enum UserEventPayload {
    Registered {
        id: UserId,
        identities: Vec<UserIdentity>,
        username: Option<Username>,
        display_name: Option<UserDisplayName>,
        bio: Option<UserBio>,
        picture: Option<UserPictureRef>,
        status: UserStatus,
    },
    IdentityLinked {
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    },
    IdentityLinkRejected {
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
        reason: UserIdentityLinkRejectionReason,
    },
    IdentityEmailChanged {
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    },
    IdentityEmailChangeRejected {
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
        reason: UserIdentityEmailChangeRejectionReason,
    },
    UsernameChanged {
        username: Username,
    },
    UsernameChangeRejected {
        username: Username,
        reason: UserUsernameChangeRejectionReason,
    },
    DisplayNameChanged {
        display_name: UserDisplayName,
    },
    DisplayNameChangeRejected {
        display_name: UserDisplayName,
        reason: UserDisplayNameChangeRejectionReason,
    },
    BioChanged {
        bio: Option<UserBio>,
    },
    BioChangeRejected {
        bio: Option<UserBio>,
        reason: UserBioChangeRejectionReason,
    },
    PictureChanged {
        picture: Option<UserPictureRef>,
        old_picture: Option<UserPictureRef>,
    },
    PictureChangeRejected {
        picture: Option<UserPictureRef>,
        reason: UserPictureChangeRejectionReason,
    },
    Activated,
    ActivateRejected {
        reason: UserStatusRejectionReason,
    },
    Inactivated,
    DeactivateRejected {
        reason: UserStatusRejectionReason,
    },
    Removed,
    RemoveRejected {
        reason: UserStatusRejectionReason,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use crate::core::Email;
    use crate::{UserBio, UserDisplayName, UserPictureRef, UserPictureUrl};

    use super::{UserEventPayload, UserId, UserIdentityProvider, UserIdentitySubject, UserStatus};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            UserEventPayload::REGISTERED,
            appletheia::domain::EventName::new("registered")
        );
        assert_eq!(
            UserEventPayload::IDENTITY_LINKED,
            appletheia::domain::EventName::new("identity_linked")
        );
        assert_eq!(
            UserEventPayload::IDENTITY_LINK_REJECTED,
            appletheia::domain::EventName::new("identity_link_rejected")
        );
        assert_eq!(
            UserEventPayload::IDENTITY_EMAIL_CHANGED,
            appletheia::domain::EventName::new("identity_email_changed")
        );
        assert_eq!(
            UserEventPayload::IDENTITY_EMAIL_CHANGE_REJECTED,
            appletheia::domain::EventName::new("identity_email_change_rejected")
        );
        assert_eq!(
            UserEventPayload::USERNAME_CHANGED,
            appletheia::domain::EventName::new("username_changed")
        );
        assert_eq!(
            UserEventPayload::USERNAME_CHANGE_REJECTED,
            appletheia::domain::EventName::new("username_change_rejected")
        );
        assert_eq!(
            UserEventPayload::DISPLAY_NAME_CHANGED,
            appletheia::domain::EventName::new("display_name_changed")
        );
        assert_eq!(
            UserEventPayload::DISPLAY_NAME_CHANGE_REJECTED,
            appletheia::domain::EventName::new("display_name_change_rejected")
        );
        assert_eq!(
            UserEventPayload::BIO_CHANGED,
            appletheia::domain::EventName::new("bio_changed")
        );
        assert_eq!(
            UserEventPayload::BIO_CHANGE_REJECTED,
            appletheia::domain::EventName::new("bio_change_rejected")
        );
        assert_eq!(
            UserEventPayload::PICTURE_CHANGED,
            appletheia::domain::EventName::new("picture_changed")
        );
        assert_eq!(
            UserEventPayload::PICTURE_CHANGE_REJECTED,
            appletheia::domain::EventName::new("picture_change_rejected")
        );
        assert_eq!(
            UserEventPayload::ACTIVATED,
            appletheia::domain::EventName::new("activated")
        );
        assert_eq!(
            UserEventPayload::ACTIVATE_REJECTED,
            appletheia::domain::EventName::new("activate_rejected")
        );
        assert_eq!(
            UserEventPayload::INACTIVATED,
            appletheia::domain::EventName::new("inactivated")
        );
        assert_eq!(
            UserEventPayload::DEACTIVATE_REJECTED,
            appletheia::domain::EventName::new("deactivate_rejected")
        );
        assert_eq!(
            UserEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
        );
        assert_eq!(
            UserEventPayload::REMOVE_REJECTED,
            appletheia::domain::EventName::new("remove_rejected")
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
            old_picture: None,
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("picture_changed"));
        assert!(value["data"]["picture"].is_object());
        assert_eq!(value["data"]["old_picture"], serde_json::Value::Null);
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
            provider: UserIdentityProvider::try_from("https://accounts.example.com")
                .expect("provider should be valid"),
            subject: UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
            email: Some(Email::try_from("alice@example.com").expect("email should be valid")),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("identity_linked"));
        assert_eq!(
            value["data"]["provider"],
            serde_json::json!("https://accounts.example.com")
        );
    }

    #[test]
    fn serializes_registered_payload_to_json() {
        let payload = UserEventPayload::Registered {
            id: UserId::new(),
            identities: Vec::new(),
            username: None,
            display_name: None,
            bio: None,
            picture: None,
            status: UserStatus::Active,
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("registered"));
        assert!(value["data"]["id"].is_string());
    }
}
