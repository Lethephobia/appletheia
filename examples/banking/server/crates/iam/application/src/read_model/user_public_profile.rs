use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};
use serde::Serialize;

use crate::projection::UserFragment;

mod user_public_profile_reader;
mod user_public_profile_reader_error;
mod user_public_profile_status;
mod user_public_profile_status_error;

pub use user_public_profile_reader::UserPublicProfileReader;
pub use user_public_profile_reader_error::UserPublicProfileReaderError;
pub use user_public_profile_status::UserPublicProfileStatus;
pub use user_public_profile_status_error::UserPublicProfileStatusError;

/// Public profile information visible to any caller.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserPublicProfile {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub bio: Option<UserBio>,
    pub picture: Option<UserPictureRef>,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for UserPublicProfile {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModel for UserPublicProfile {
    const NAME: ReadModelName = ReadModelName::new("user_public_profile");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        Ok(vec![SerializedPartition::try_from_fragment_key::<
            UserFragment,
        >(&self.id)?])
    }
}
