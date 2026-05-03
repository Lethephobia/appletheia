use banking_iam_domain::{Email, UserId, UserIdentityProvider, UserIdentitySubject};

/// Attributes required to upsert a normalized user identity projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserIdentityProjectionUpsert {
    pub user_id: UserId,
    pub provider: UserIdentityProvider,
    pub subject: UserIdentitySubject,
    pub email: Option<Email>,
}
