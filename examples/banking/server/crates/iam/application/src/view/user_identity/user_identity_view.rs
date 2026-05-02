use banking_iam_domain::{Email, UserId, UserIdentityProvider, UserIdentitySubject};

/// Represents a normalized user identity view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserIdentityView {
    pub user_id: UserId,
    pub provider: UserIdentityProvider,
    pub subject: UserIdentitySubject,
    pub email: Option<Email>,
}
