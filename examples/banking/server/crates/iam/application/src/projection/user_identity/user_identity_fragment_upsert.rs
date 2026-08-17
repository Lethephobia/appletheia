use banking_iam_domain::{UserId, UserIdentityProvider, UserIdentitySubject};
use banking_shared_kernel_domain::contact::Email;

/// Values used to create or replace a user identity fragment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserIdentityFragmentUpsert {
    pub user_id: UserId,
    pub provider: UserIdentityProvider,
    pub subject: UserIdentitySubject,
    pub email: Option<Email>,
}
