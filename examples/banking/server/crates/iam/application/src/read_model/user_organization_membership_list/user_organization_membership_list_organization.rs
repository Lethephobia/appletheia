use serde::Serialize;

use appletheia::application::read_model::ReadModelObservation;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

/// Organization profile embedded in a user organization membership list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserOrganizationMembershipListOrganization {
    pub organization_id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub observation: ReadModelObservation,
}
