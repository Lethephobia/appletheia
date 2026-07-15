use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

use banking_shared_kernel_application::read_model::ReadModelObservation;

/// Organization owner shown in an owned account list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountListOwnerOrganization {
    pub id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub observation: ReadModelObservation,
}
