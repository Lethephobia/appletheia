use appletheia::domain::{EventId, EventOccurredAt};
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationPictureRef, OrganizationWebsiteUrl,
};
use banking_shared_kernel_application::read_model::ReadModelObservation;

mod organization_management_info_owner;
mod organization_management_info_owner_upsert;
mod organization_management_info_reader;
mod organization_management_info_reader_error;
mod organization_management_info_upsert;
mod organization_management_info_writer;
mod organization_management_info_writer_error;

pub use organization_management_info_owner::OrganizationManagementInfoOwner;
pub use organization_management_info_owner_upsert::OrganizationManagementInfoOwnerUpsert;
pub use organization_management_info_reader::OrganizationManagementInfoReader;
pub use organization_management_info_reader_error::OrganizationManagementInfoReaderError;
pub use organization_management_info_upsert::OrganizationManagementInfoUpsert;
pub use organization_management_info_writer::OrganizationManagementInfoWriter;
pub use organization_management_info_writer_error::OrganizationManagementInfoWriterError;

/// Organization information visible to its administrators.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationManagementInfo {
    pub id: OrganizationId,
    pub owner: OrganizationManagementInfoOwner,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub description: Option<OrganizationDescription>,
    pub website_url: Option<OrganizationWebsiteUrl>,
    pub picture: Option<OrganizationPictureRef>,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl OrganizationManagementInfo {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        ReadModelObservation::collect_event_ids(
            self.observation
                .event_ids()
                .chain(self.owner.observation.event_ids()),
        )
    }
}
