use banking_iam_domain::{Email, UserId, UserIdentityProvider, UserIdentitySubject};

/// Attributes required to upsert a normalized user identity view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserIdentityViewUpsert {
    pub user_id: UserId,
    pub provider: UserIdentityProvider,
    pub subject: UserIdentitySubject,
    pub email: Option<Email>,
}
