use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListOwnerUserUpsert {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
}
