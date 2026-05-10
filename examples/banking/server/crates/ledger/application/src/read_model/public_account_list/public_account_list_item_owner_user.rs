use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

/// User owner fields exposed in public account list items.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicAccountListItemOwnerUser {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
}
