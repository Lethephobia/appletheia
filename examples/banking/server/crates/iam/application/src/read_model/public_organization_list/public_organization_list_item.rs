use serde::Serialize;

use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

/// Read model for one public organization list row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PublicOrganizationListItem {
    pub organization_id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub created_at: EventOccurredAt,
}
