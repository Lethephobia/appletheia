use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};
use serde::Serialize;

use crate::projection::{UserFragment, UserIdentityFragment, UserIdentityFragmentKey};

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
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for UserPrivateInfo {
    fn observations(&self) -> Vec<ReadModelObservation> {
        std::iter::once(self.observation)
            .chain(self.identities.iter().map(|identity| identity.observation))
            .collect()
    }
}

impl ReadModel for UserPrivateInfo {
    const NAME: ReadModelName = ReadModelName::new("user_private_info");

    fn partitions(&self) -> Result<Vec<SerializedPartition>, SerializedPartitionError> {
        let mut partitions = vec![SerializedPartition::try_from_fragment_key::<UserFragment>(
            &self.id,
        )?];
        for identity in &self.identities {
            let identity_key = UserIdentityFragmentKey {
                user_id: self.id,
                provider: identity.provider.clone(),
                subject: identity.subject.clone(),
            };
            partitions.push(SerializedPartition::try_from_fragment_key::<
                UserIdentityFragment,
            >(&identity_key)?);
        }
        Ok(partitions)
    }
}
