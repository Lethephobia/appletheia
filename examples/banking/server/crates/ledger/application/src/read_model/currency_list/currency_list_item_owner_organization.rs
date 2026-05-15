use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

use crate::read_model::ReadModelObservation;

/// Organization owner shown in a currency list item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyListItemOwnerOrganization {
    pub id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub observation: ReadModelObservation,
}
