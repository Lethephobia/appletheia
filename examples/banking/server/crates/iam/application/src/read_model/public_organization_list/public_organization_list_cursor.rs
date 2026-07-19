use appletheia::domain::EventOccurredAt;
use banking_iam_domain::OrganizationId;

/// Cursor for public organization list pagination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PublicOrganizationListCursor {
    pub created_at: EventOccurredAt,
    pub organization_id: OrganizationId,
}
