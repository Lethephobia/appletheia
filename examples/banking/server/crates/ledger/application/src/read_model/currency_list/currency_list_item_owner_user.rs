use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

/// User owner shown in a currency list item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyListItemOwnerUser {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
}
