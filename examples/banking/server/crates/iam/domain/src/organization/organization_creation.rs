use super::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationOwner,
    OrganizationPictureRef, OrganizationWebsiteUrl,
};

/// Describes an organization creation request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrganizationCreation {
    pub owner: OrganizationOwner,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub description: Option<OrganizationDescription>,
    pub website_url: Option<OrganizationWebsiteUrl>,
    pub picture: Option<OrganizationPictureRef>,
}

impl OrganizationCreation {
    pub(super) fn into_parts(
        self,
    ) -> (
        OrganizationOwner,
        OrganizationHandle,
        OrganizationDisplayName,
        Option<OrganizationDescription>,
        Option<OrganizationWebsiteUrl>,
        Option<OrganizationPictureRef>,
    ) {
        (
            self.owner,
            self.handle,
            self.display_name,
            self.description,
            self.website_url,
            self.picture,
        )
    }
}
