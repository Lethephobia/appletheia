use appletheia::query;
use banking_iam_domain::UserId;

/// Query parameters for public user profile reads.
#[query(name = "user_public_profile")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPublicProfileQuery {
    pub user_id: UserId,
}
