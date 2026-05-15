use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

use crate::read_model::ReadModelObservation;

/// Organization owner shown in an owned account transaction list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListOwnerOrganization {
    pub id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub observation: ReadModelObservation,
}
