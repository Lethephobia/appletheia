use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationOwner, OrganizationPictureRef, OrganizationWebsiteUrl,
};

/// Attributes required to upsert a normalized organization view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationViewUpsert {
    pub id: OrganizationId,
    pub owner: OrganizationOwner,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub description: Option<OrganizationDescription>,
    pub website_url: Option<OrganizationWebsiteUrl>,
    pub picture: Option<OrganizationPictureRef>,
}
