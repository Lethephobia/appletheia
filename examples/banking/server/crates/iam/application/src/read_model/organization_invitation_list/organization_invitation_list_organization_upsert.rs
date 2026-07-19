use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

/// Describes an organization invitation list organization snapshot upsert.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationInvitationListOrganizationUpsert {
    pub organization_id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
}
