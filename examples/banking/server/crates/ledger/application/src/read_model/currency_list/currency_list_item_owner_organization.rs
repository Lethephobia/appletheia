use serde::Serialize;

use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

use appletheia::application::read_model::ReadModelObservation;

/// Organization owner shown in a currency list item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CurrencyListItemOwnerOrganization {
    pub id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub observation: ReadModelObservation,
}
