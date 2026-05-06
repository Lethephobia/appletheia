use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

/// User owner shown for a counterparty account.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListItemCounterpartyAccountOwnerUser {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
}
