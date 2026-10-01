use appletheia::application::read_model::{ReadModel, ReadModelName};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationPictureRef, OrganizationWebsiteUrl,
};
use serde::Serialize;

mod organization_internal_info_reader;
mod organization_internal_info_reader_error;

pub use organization_internal_info_reader::OrganizationInternalInfoReader;
pub use organization_internal_info_reader_error::OrganizationInternalInfoReaderError;

/// Organization information visible to its members.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationInternalInfo {
    pub id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub description: Option<OrganizationDescription>,
    pub website_url: Option<OrganizationWebsiteUrl>,
    pub picture: Option<OrganizationPictureRef>,
    pub created_at: EventOccurredAt,
}

impl ReadModel for OrganizationInternalInfo {
    const NAME: ReadModelName = ReadModelName::new("organization_internal_info");
}
