use serde::{Deserialize, Serialize};

use banking_iam_application::OrganizationFragment;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

use appletheia::application::read_model::ReadModelObservation;

/// Organization owner shown in an owned account transaction list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OwnedAccountTransactionListOwnerOrganizationPart {
    pub id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub observation: ReadModelObservation,
}

impl From<OrganizationFragment> for OwnedAccountTransactionListOwnerOrganizationPart {
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
