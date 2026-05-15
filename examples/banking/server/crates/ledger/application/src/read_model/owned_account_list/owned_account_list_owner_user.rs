use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

use crate::read_model::ReadModelObservation;

/// User owner shown in an owned account list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListOwnerUser {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub observation: ReadModelObservation,
}
