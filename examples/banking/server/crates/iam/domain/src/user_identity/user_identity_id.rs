use appletheia::aggregate_id;
use appletheia::domain::AggregateId;
use uuid::{Uuid, Version};

use super::{UserIdentityIdError, UserIdentityProvider, UserIdentitySubject};

/// Identifies a `UserIdentity` aggregate.
#[aggregate_id(error = UserIdentityIdError, validate = validate_user_identity_id)]
pub struct UserIdentityId(Uuid);

impl UserIdentityId {
    /// Creates a deterministic user-identity ID from `provider` and `subject`.
    pub fn new(provider: &UserIdentityProvider, subject: &UserIdentitySubject) -> Self {
        Self(Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            format!(
                "banking:iam:user_identity:{}:{}",
                provider.value(),
                subject.value(),
            )
            .as_bytes(),
        ))
    }
}

fn validate_user_identity_id(value: Uuid) -> Result<(), UserIdentityIdError> {
    match value.get_version() {
        Some(Version::Sha1) => Ok(()),
        _ => Err(UserIdentityIdError::NotUuidV5(value)),
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::{Uuid, Version};

    use super::{UserIdentityId, UserIdentityIdError, UserIdentityProvider, UserIdentitySubject};

    #[test]
    fn new_creates_deterministic_user_identity_id() {
        let provider = UserIdentityProvider::try_from("https://accounts.example.com")
            .expect("provider should be valid");
        let subject = UserIdentitySubject::try_from("user-123").expect("subject should be valid");
        let identity_id = UserIdentityId::new(&provider, &subject);
        let same_identity_id = UserIdentityId::new(&provider, &subject);

        assert!(!identity_id.value().is_nil());
        assert_eq!(identity_id.value().get_version(), Some(Version::Sha1));
        assert_eq!(identity_id, same_identity_id);
    }

    #[test]
    fn new_changes_when_provider_or_subject_changes() {
        let provider = UserIdentityProvider::try_from("https://accounts.example.com")
            .expect("provider should be valid");
        let other_provider = UserIdentityProvider::try_from("https://login.example.com")
            .expect("provider should be valid");
        let subject = UserIdentitySubject::try_from("user-123").expect("subject should be valid");
        let other_subject =
            UserIdentitySubject::try_from("user-456").expect("subject should be valid");

        assert_ne!(
            UserIdentityId::new(&provider, &subject),
            UserIdentityId::new(&other_provider, &subject)
        );
        assert_ne!(
            UserIdentityId::new(&provider, &subject),
            UserIdentityId::new(&provider, &other_subject)
        );
    }

    #[test]
    fn try_from_uuid_accepts_uuid_v5() {
        let uuid = Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            b"banking:iam:user_identity:https://accounts.example.com:user-123",
        );
        let identity_id = UserIdentityId::try_from_uuid(uuid).expect("uuidv5 should be accepted");

        assert_eq!(identity_id.value(), uuid);
    }

    #[test]
    fn try_from_uuid_rejects_non_uuid_v5() {
        let uuid = Uuid::nil();

        match UserIdentityId::try_from_uuid(uuid) {
            Err(UserIdentityIdError::NotUuidV5(returned)) => assert_eq!(returned, uuid),
            other => panic!("expected NotUuidV5 error, got {other:?}"),
        }
    }
}
