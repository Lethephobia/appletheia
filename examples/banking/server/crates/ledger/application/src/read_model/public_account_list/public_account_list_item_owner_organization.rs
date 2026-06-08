use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

use banking_shared_kernel_application::read_model::ReadModelObservation;

/// Organization owner fields exposed in public account list items.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicAccountListItemOwnerOrganization {
    pub id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub observation: ReadModelObservation,
}
