use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

use crate::read_model::ReadModelObservation;

/// Organization snapshot shown in user-private information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPrivateInfoOrganization {
    pub id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub observation: ReadModelObservation,
}
