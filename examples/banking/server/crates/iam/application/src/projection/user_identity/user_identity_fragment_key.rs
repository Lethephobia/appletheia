use banking_iam_domain::{UserId, UserIdentityProvider, UserIdentitySubject};
use serde::{Deserialize, Serialize};

/// Identifies one stored user identity fragment.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserIdentityFragmentKey {
    pub user_id: UserId,
    pub provider: UserIdentityProvider,
    pub subject: UserIdentitySubject,
}
