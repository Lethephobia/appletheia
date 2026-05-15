use appletheia::application::event::EventSequence;
use appletheia::domain::EventId;
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedAccountTransactionListOwnerOrganizationUpsert {
    pub id: OrganizationId,
    pub handle: OrganizationHandle,
    pub display_name: OrganizationDisplayName,
    pub picture: Option<OrganizationPictureRef>,
    pub event_id: EventId,
    pub event_sequence: EventSequence,
    pub occurred_at: EventOccurredAt,
}
