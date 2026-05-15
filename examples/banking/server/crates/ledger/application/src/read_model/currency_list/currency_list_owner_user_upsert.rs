use appletheia::application::event::EventSequence;
use appletheia::domain::EventId;
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyListOwnerUserUpsert {
    pub id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub event_id: EventId,
    pub event_sequence: EventSequence,
    pub occurred_at: EventOccurredAt,
}
