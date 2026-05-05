use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{User, UserEventPayload};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

/// Projector specification for transfer recipient list item read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TransferRecipientListItemProjectorSpec;

impl ProjectorSpec for TransferRecipientListItemProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("transfer_recipient_list_item"),
        Subscription::AnyOf(&[
            EventSelector::new(User::TYPE, UserEventPayload::REGISTERED),
            EventSelector::new(User::TYPE, UserEventPayload::USERNAME_CHANGED),
            EventSelector::new(User::TYPE, UserEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new(User::TYPE, UserEventPayload::PICTURE_CHANGED),
            EventSelector::new(User::TYPE, UserEventPayload::ACTIVATED),
            EventSelector::new(User::TYPE, UserEventPayload::INACTIVATED),
            EventSelector::new(User::TYPE, UserEventPayload::REMOVED),
            EventSelector::new(Account::TYPE, AccountEventPayload::OPENED),
            EventSelector::new(Account::TYPE, AccountEventPayload::OWNERSHIP_TRANSFERRED),
            EventSelector::new(Account::TYPE, AccountEventPayload::FROZEN),
            EventSelector::new(Account::TYPE, AccountEventPayload::THAWED),
            EventSelector::new(Account::TYPE, AccountEventPayload::CLOSED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::DEFINED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::SYMBOL_CHANGED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::NAME_CHANGED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::REMOVED),
        ]),
    );
}
