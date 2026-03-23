mod user_identity_error;
mod user_identity_event_payload;
mod user_identity_event_payload_error;
mod user_identity_id;
mod user_identity_id_error;
mod user_identity_provider;
mod user_identity_provider_error;
mod user_identity_state;
mod user_identity_state_error;
mod user_identity_subject;
mod user_identity_subject_error;

pub use user_identity_error::UserIdentityError;
pub use user_identity_event_payload::UserIdentityEventPayload;
pub use user_identity_event_payload_error::UserIdentityEventPayloadError;
pub use user_identity_id::UserIdentityId;
pub use user_identity_id_error::UserIdentityIdError;
pub use user_identity_provider::UserIdentityProvider;
pub use user_identity_provider_error::UserIdentityProviderError;
pub use user_identity_state::UserIdentityState;
pub use user_identity_state_error::UserIdentityStateError;
pub use user_identity_subject::UserIdentitySubject;
pub use user_identity_subject_error::UserIdentitySubjectError;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::core::Email;
use crate::user::UserId;

/// Represents the `UserIdentity` aggregate root.
#[aggregate(type = "user_identity", error = UserIdentityError)]
pub struct UserIdentity {
    core: AggregateCore<UserIdentityState, UserIdentityEventPayload>,
}

impl UserIdentity {
    /// Creates a new external identity.
    pub fn create(
        &mut self,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    ) -> Result<(), UserIdentityError> {
        if self.state().is_some() {
            return Err(UserIdentityError::AlreadyCreated);
        }

        self.append_event(UserIdentityEventPayload::Created {
            id: UserIdentityId::new(&provider, &subject),
            provider,
            subject,
            email,
        })
    }

    /// Links the external identity to a user.
    pub fn link_to_user(&mut self, user_id: UserId) -> Result<(), UserIdentityError> {
        match self.state_required()?.user_id() {
            Some(current_user_id) if current_user_id == &user_id => Ok(()),
            Some(_) => Err(UserIdentityError::AlreadyLinkedToUser),
            None => self.append_event(UserIdentityEventPayload::LinkedToUser { user_id }),
        }
    }

    /// Updates the current email snapshot.
    pub fn change_email(&mut self, email: Option<Email>) -> Result<(), UserIdentityError> {
        if self.state_required()?.email() == email.as_ref() {
            return Ok(());
        }

        self.append_event(UserIdentityEventPayload::EmailChanged { email })
    }
}

impl AggregateApply<UserIdentityEventPayload, UserIdentityError> for UserIdentity {
    fn apply(&mut self, payload: &UserIdentityEventPayload) -> Result<(), UserIdentityError> {
        match payload {
            UserIdentityEventPayload::Created {
                id,
                provider,
                subject,
                email,
            } => {
                self.set_state(Some(UserIdentityState::new(
                    *id,
                    provider.clone(),
                    subject.clone(),
                    email.clone(),
                )));
            }
            UserIdentityEventPayload::LinkedToUser { user_id } => {
                self.state_required_mut()?.set_user_id(*user_id);
            }
            UserIdentityEventPayload::EmailChanged { email } => {
                self.state_required_mut()?.set_email(email.clone());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateState, Event, EventPayload};

    use crate::core::Email;
    use crate::user::UserId;

    use super::{
        UserIdentity, UserIdentityEventPayload, UserIdentityId, UserIdentityProvider,
        UserIdentitySubject,
    };

    #[test]
    fn link_initializes_state_and_records_event() {
        let provider = UserIdentityProvider::try_from("https://accounts.example.com")
            .expect("provider should be valid");
        let subject = UserIdentitySubject::try_from("user-123").expect("subject should be valid");
        let email = Some(Email::try_from("alice@example.com").expect("email should be valid"));
        let mut identity = UserIdentity::default();

        identity
            .create(provider.clone(), subject.clone(), email.clone())
            .expect("create should succeed");

        let state = identity.state().expect("state should exist");
        assert_eq!(
            state.id(),
            identity.aggregate_id().expect("aggregate id should exist")
        );
        assert_eq!(state.user_id(), None);
        assert_eq!(state.provider(), &provider);
        assert_eq!(state.subject(), &subject);
        assert_eq!(state.email(), email.as_ref());
        assert_eq!(identity.uncommitted_events().len(), 1);
        assert_eq!(
            identity.uncommitted_events()[0].payload().name(),
            UserIdentityEventPayload::CREATED
        );
    }

    #[test]
    fn changing_to_same_email_is_a_no_op() {
        let provider = UserIdentityProvider::try_from("https://accounts.example.com")
            .expect("provider should be valid");
        let subject = UserIdentitySubject::try_from("user-123").expect("subject should be valid");
        let email = Some(Email::try_from("alice@example.com").expect("email should be valid"));
        let mut identity = UserIdentity::default();
        identity
            .create(provider, subject, email.clone())
            .expect("create should succeed");

        identity
            .change_email(email)
            .expect("no-op email change should succeed");

        assert_eq!(identity.uncommitted_events().len(), 1);
    }

    #[test]
    fn change_email_appends_event_and_updates_state() {
        let mut identity = UserIdentity::default();
        identity
            .create(
                UserIdentityProvider::try_from("https://accounts.example.com")
                    .expect("provider should be valid"),
                UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
                Some(Email::try_from("alice@example.com").expect("email should be valid")),
            )
            .expect("create should succeed");

        let changed_email =
            Some(Email::try_from("alice@bank.example").expect("email should be valid"));
        identity
            .change_email(changed_email.clone())
            .expect("email change should succeed");

        let state = identity.state().expect("state should exist");
        assert_eq!(state.email(), changed_email.as_ref());
        assert_eq!(identity.uncommitted_events().len(), 2);
        assert_eq!(
            identity.uncommitted_events()[1].payload().name(),
            UserIdentityEventPayload::EMAIL_CHANGED
        );
    }

    #[test]
    fn link_to_user_appends_event_and_updates_state() {
        let user_id = UserId::new();
        let mut identity = UserIdentity::default();
        identity
            .create(
                UserIdentityProvider::try_from("https://accounts.example.com")
                    .expect("provider should be valid"),
                UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
                None,
            )
            .expect("create should succeed");

        identity
            .link_to_user(user_id)
            .expect("link to user should succeed");

        let state = identity.state().expect("state should exist");
        assert_eq!(state.user_id(), Some(&user_id));
        assert_eq!(identity.uncommitted_events().len(), 2);
        assert_eq!(
            identity.uncommitted_events()[1].payload().name(),
            UserIdentityEventPayload::LINKED_TO_USER
        );
    }

    #[test]
    fn replay_events_rebuilds_state() {
        let user_id = UserId::new();
        let provider = UserIdentityProvider::try_from("https://accounts.example.com")
            .expect("provider should be valid");
        let subject = UserIdentitySubject::try_from("user-123").expect("subject should be valid");
        let id = UserIdentityId::new(&provider, &subject);
        let created = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(1).expect("version should be valid"),
            UserIdentityEventPayload::Created {
                id,
                provider,
                subject,
                email: Some(Email::try_from("alice@example.com").expect("email should be valid")),
            },
        );
        let linked_to_user = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            UserIdentityEventPayload::LinkedToUser { user_id },
        );
        let email_changed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(3).expect("version should be valid"),
            UserIdentityEventPayload::EmailChanged {
                email: Some(Email::try_from("alice@bank.example").expect("email should be valid")),
            },
        );
        let mut identity = UserIdentity::default();

        identity
            .replay_events(vec![created, linked_to_user, email_changed], None)
            .expect("events should replay");

        let state = identity.state().expect("state should exist");
        assert_eq!(
            state.email().expect("email should exist").value(),
            "alice@bank.example"
        );
        assert_eq!(state.user_id(), Some(&user_id));
        assert_eq!(identity.version().value(), 3);
        assert!(identity.uncommitted_events().is_empty());
    }

    #[test]
    fn create_rejects_already_created_identity() {
        let mut identity = UserIdentity::default();
        identity
            .create(
                UserIdentityProvider::try_from("https://accounts.example.com")
                    .expect("provider should be valid"),
                UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
                None,
            )
            .expect("create should succeed");

        let error = identity
            .create(
                UserIdentityProvider::try_from("https://accounts.example.com")
                    .expect("provider should be valid"),
                UserIdentitySubject::try_from("user-456").expect("subject should be valid"),
                None,
            )
            .expect_err("duplicate create should fail");

        assert!(matches!(error, super::UserIdentityError::AlreadyCreated));
    }

    #[test]
    fn link_to_user_rejects_already_linked_identity() {
        let mut identity = UserIdentity::default();
        identity
            .create(
                UserIdentityProvider::try_from("https://accounts.example.com")
                    .expect("provider should be valid"),
                UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
                None,
            )
            .expect("create should succeed");
        identity
            .link_to_user(UserId::new())
            .expect("link to user should succeed");

        let error = identity
            .link_to_user(UserId::new())
            .expect_err("duplicate link to user should fail");

        assert!(matches!(
            error,
            super::UserIdentityError::AlreadyLinkedToUser
        ));
    }
}
