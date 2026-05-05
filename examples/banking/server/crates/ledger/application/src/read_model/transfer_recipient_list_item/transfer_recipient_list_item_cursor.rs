use appletheia::domain::EventOccurredAt;
use banking_iam_domain::UserId;

/// Cursor for transfer recipient list pagination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TransferRecipientListItemCursor {
    pub created_at: EventOccurredAt,
    pub user_id: UserId,
}
