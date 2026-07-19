use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

/// Describes a public organization list item upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicOrganizationListUpsert {
    pub id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
}
