use super::UserIdentityRegistration;

/// Describes a user registration request.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UserRegistration {
    pub initial_identity: Option<UserIdentityRegistration>,
}
