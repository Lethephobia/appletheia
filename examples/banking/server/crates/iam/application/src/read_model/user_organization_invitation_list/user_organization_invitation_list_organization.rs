use serde::Serialize;

use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

/// Organization profile embedded in a user organization invitation list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserOrganizationInvitationListOrganization {
    pub organization_id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
}
