use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

/// Describes an organization join request list organization snapshot upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationJoinRequestListOrganizationUpsert {
    pub organization_id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
}
