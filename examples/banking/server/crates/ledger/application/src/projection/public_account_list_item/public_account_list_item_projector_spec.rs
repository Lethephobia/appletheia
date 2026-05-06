use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

/// Projector specification for public account list item read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PublicAccountListItemProjectorSpec;

impl ProjectorSpec for PublicAccountListItemProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("public_account_list_item"),
        Subscription::AnyOf(&[
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
