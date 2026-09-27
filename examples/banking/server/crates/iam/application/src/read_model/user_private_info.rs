use appletheia::application::read_model::{ReadModel, ReadModelName};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};
use serde::Serialize;

mod user_private_info_identity;
mod user_private_info_reader;
mod user_private_info_reader_error;
mod user_private_info_status;
mod user_private_info_status_error;

pub use user_private_info_identity::UserPrivateInfoIdentity;
pub use user_private_info_reader::UserPrivateInfoReader;
pub use user_private_info_reader_error::UserPrivateInfoReaderError;
pub use user_private_info_status::UserPrivateInfoStatus;
pub use user_private_info_status_error::UserPrivateInfoStatusError;

/// Read model containing private information for the owning user.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserPrivateInfo {
    pub id: UserId,
    pub identities: Vec<UserPrivateInfoIdentity>,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub bio: Option<UserBio>,
    pub picture: Option<UserPictureRef>,
    pub status: UserPrivateInfoStatus,
    pub created_at: EventOccurredAt,
}

impl ReadModel for UserPrivateInfo {
    const NAME: ReadModelName = ReadModelName::new("user_private_info");
}
