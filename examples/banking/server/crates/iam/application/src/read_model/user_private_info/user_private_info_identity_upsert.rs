use banking_iam_domain::{UserId, UserIdentityProvider, UserIdentitySubject, core::Email};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPrivateInfoIdentityUpsert {
    pub user_id: UserId,
    pub provider: UserIdentityProvider,
    pub subject: UserIdentitySubject,
    pub email: Option<Email>,
}
