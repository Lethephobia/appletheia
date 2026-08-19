use serde::Serialize;

use appletheia::application::read_model::ReadModelObservation;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

/// Organization snapshot shown in user-private information.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserPrivateInfoOrganization {
    pub id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub observation: ReadModelObservation,
}
