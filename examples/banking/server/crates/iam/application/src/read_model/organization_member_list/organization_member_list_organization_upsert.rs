use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef, UserId,
};

/// Describes an organization member list organization snapshot upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationMemberListOrganizationUpsert {
    pub organization_id: OrganizationId,
    pub owner_user_id: UserId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
}
