use appletheia::domain::{EventId, EventOccurredAt};
use banking_iam_domain::{UserBio, UserDisplayName, UserId, UserPictureRef, Username};

use super::ReadModelObservation;

mod user_private_info_identity;
mod user_private_info_identity_upsert;
mod user_private_info_organization;
mod user_private_info_organization_membership;
mod user_private_info_organization_membership_upsert;
mod user_private_info_organization_upsert;
mod user_private_info_reader;
mod user_private_info_reader_error;
mod user_private_info_status;
mod user_private_info_status_error;
mod user_private_info_user_upsert;
mod user_private_info_writer;
mod user_private_info_writer_error;

pub use user_private_info_identity::UserPrivateInfoIdentity;
pub use user_private_info_identity_upsert::UserPrivateInfoIdentityUpsert;
pub use user_private_info_organization::UserPrivateInfoOrganization;
pub use user_private_info_organization_membership::UserPrivateInfoOrganizationMembership;
pub use user_private_info_organization_membership_upsert::UserPrivateInfoOrganizationMembershipUpsert;
pub use user_private_info_organization_upsert::UserPrivateInfoOrganizationUpsert;
pub use user_private_info_reader::UserPrivateInfoReader;
pub use user_private_info_reader_error::UserPrivateInfoReaderError;
pub use user_private_info_status::UserPrivateInfoStatus;
pub use user_private_info_status_error::UserPrivateInfoStatusError;
pub use user_private_info_user_upsert::UserPrivateInfoUserUpsert;
pub use user_private_info_writer::UserPrivateInfoWriter;
pub use user_private_info_writer_error::UserPrivateInfoWriterError;

/// Read model containing private information for the owning user.
#[derive(Clone, Debug, Eq, PartialEq)]
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

impl UserPrivateInfo {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observation
                .event_ids()
                .chain(
                    self.identities
                        .iter()
                        .flat_map(|identity| identity.observation.event_ids()),
                )
                .chain(
                    self.organization_memberships
                        .iter()
                        .flat_map(UserPrivateInfoOrganizationMembership::observed_event_ids),
                ),
        )
    }
}
