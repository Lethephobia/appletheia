use appletheia::application::read_model::{ReadModel, ReadModelName};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationPictureRef, OrganizationWebsiteUrl,
};
use serde::Serialize;

mod organization_management_info_owner;
mod organization_management_info_reader;
mod organization_management_info_reader_error;

pub use organization_management_info_owner::OrganizationManagementInfoOwner;
pub use organization_management_info_reader::OrganizationManagementInfoReader;
pub use organization_management_info_reader_error::OrganizationManagementInfoReaderError;

/// Organization information visible to its administrators.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrganizationManagementInfo {
    pub id: OrganizationId,
    pub owner: OrganizationManagementInfoOwner,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub description: Option<OrganizationDescription>,
    pub website_url: Option<OrganizationWebsiteUrl>,
    pub picture: Option<OrganizationPictureRef>,
    pub created_at: EventOccurredAt,
}

impl ReadModel for OrganizationManagementInfo {
    const NAME: ReadModelName = ReadModelName::new("organization_management_info");
}
