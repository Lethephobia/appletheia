use appletheia::domain::{EventId, EventOccurredAt};
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationPictureRef, OrganizationWebsiteUrl,
};
use banking_shared_kernel_application::read_model::ReadModelObservation;

mod organization_internal_info_reader;
mod organization_internal_info_reader_error;
mod organization_internal_info_upsert;
mod organization_internal_info_writer;
mod organization_internal_info_writer_error;

pub use organization_internal_info_reader::OrganizationInternalInfoReader;
pub use organization_internal_info_reader_error::OrganizationInternalInfoReaderError;
pub use organization_internal_info_upsert::OrganizationInternalInfoUpsert;
pub use organization_internal_info_writer::OrganizationInternalInfoWriter;
pub use organization_internal_info_writer_error::OrganizationInternalInfoWriterError;

/// Organization information visible to its members.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationInternalInfo {
    pub id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub description: Option<OrganizationDescription>,
    pub website_url: Option<OrganizationWebsiteUrl>,
    pub picture: Option<OrganizationPictureRef>,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl OrganizationInternalInfo {
    pub fn observed_event_ids(&self) -> Vec<EventId> {
        self.observation.event_ids().collect()
    }
}
