use banking_iam_domain::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationPictureRef, OrganizationWebsiteUrl, UserId,
};

/// Describes an organization-management information upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationManagementInfoUpsert {
    pub id: OrganizationId,
    pub owner_user_id: UserId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub description: Option<OrganizationDescription>,
    pub website_url: Option<OrganizationWebsiteUrl>,
    pub picture: Option<OrganizationPictureRef>,
}
