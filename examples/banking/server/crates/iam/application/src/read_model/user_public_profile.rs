use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};

mod user_public_profile_reader;
mod user_public_profile_reader_error;
mod user_public_profile_status;
mod user_public_profile_status_error;
mod user_public_profile_user_upsert;
mod user_public_profile_writer;
mod user_public_profile_writer_error;

pub use user_public_profile_reader::UserPublicProfileReader;
pub use user_public_profile_reader_error::UserPublicProfileReaderError;
pub use user_public_profile_status::UserPublicProfileStatus;
pub use user_public_profile_status_error::UserPublicProfileStatusError;
pub use user_public_profile_user_upsert::UserPublicProfileUserUpsert;
pub use user_public_profile_writer::UserPublicProfileWriter;
pub use user_public_profile_writer_error::UserPublicProfileWriterError;

/// Public profile information visible to any caller.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPublicProfile {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub bio: Option<UserBio>,
    pub picture: Option<UserPictureRef>,
    pub created_at: EventOccurredAt,
}
