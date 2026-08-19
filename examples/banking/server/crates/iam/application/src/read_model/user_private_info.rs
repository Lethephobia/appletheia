use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    SerializedPartition, SerializedPartitionError,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};
use serde::Serialize;

use crate::projection::{
    OrganizationFragment, OrganizationMembershipFragment, OrganizationMembershipFragmentKey,
    UserFragment, UserIdentityFragment, UserIdentityFragmentKey,
};

mod user_private_info_identity;
mod user_private_info_organization;
mod user_private_info_organization_membership;
mod user_private_info_reader;
mod user_private_info_reader_error;
mod user_private_info_status;
mod user_private_info_status_error;

pub use user_private_info_identity::UserPrivateInfoIdentity;
pub use user_private_info_organization::UserPrivateInfoOrganization;
pub use user_private_info_organization_membership::UserPrivateInfoOrganizationMembership;
pub use user_private_info_reader::UserPrivateInfoReader;
pub use user_private_info_reader_error::UserPrivateInfoReaderError;
pub use user_private_info_status::UserPrivateInfoStatus;
pub use user_private_info_status_error::UserPrivateInfoStatusError;

/// Read model containing private information for the owning user.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserPrivateInfo {
    pub id: UserId,
    pub identities: Vec<UserPrivateInfoIdentity>,
    pub organization_memberships: Vec<UserPrivateInfoOrganizationMembership>,
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
            .chain(self.organization_memberships.iter().flat_map(|membership| {
                [membership.observation, membership.organization.observation]
            }))
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
        for membership in &self.organization_memberships {
            let membership_key = OrganizationMembershipFragmentKey {
                user_id: self.id,
                organization_id: membership.organization.id,
            };
            partitions.push(SerializedPartition::try_from_fragment_key::<
                OrganizationMembershipFragment,
            >(&membership_key)?);
            partitions.push(SerializedPartition::try_from_fragment_key::<
                OrganizationFragment,
            >(&membership.organization.id)?);
        }
        Ok(partitions)
    }
}
