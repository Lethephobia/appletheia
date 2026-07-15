use appletheia::query;
use banking_iam_domain::UserId;

/// Query parameters for the owning user's private information.
#[query(name = "user_private_info")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPrivateInfoQuery {
    pub user_id: UserId,
}
