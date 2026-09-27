use serde::Serialize;

use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

/// Organization profile embedded in a user organization join request list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserOrganizationJoinRequestListOrganization {
    pub organization_id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
}
