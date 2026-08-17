use serde::{Deserialize, Serialize};

use banking_iam_application::OrganizationFragment;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

use appletheia::application::read_model::ReadModelObservation;

/// Organization owner fields exposed in public account list items.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicAccountListItemOwnerOrganizationPart {
    pub id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub observation: ReadModelObservation,
}

impl From<OrganizationFragment> for PublicAccountListItemOwnerOrganizationPart {
    fn from(fragment: OrganizationFragment) -> Self {
        Self {
            id: fragment.id,
            handle: fragment.handle,
            display_name: fragment.display_name,
            picture: fragment.picture,
            observation: fragment.observation,
        }
    }
}
