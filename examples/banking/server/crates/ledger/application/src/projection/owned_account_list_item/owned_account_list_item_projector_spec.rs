use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

/// Projector specification for owned account list item read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OwnedAccountListItemProjectorSpec;

impl ProjectorSpec for OwnedAccountListItemProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("owned_account_list_item"),
        Subscription::AnyOf(&[
            EventSelector::new(Account::TYPE, AccountEventPayload::OPENED),
            EventSelector::new(Account::TYPE, AccountEventPayload::OWNERSHIP_TRANSFERRED),
            EventSelector::new(Account::TYPE, AccountEventPayload::NAME_CHANGED),
            EventSelector::new(Account::TYPE, AccountEventPayload::DEPOSITED),
            EventSelector::new(Account::TYPE, AccountEventPayload::WITHDRAWN),
            EventSelector::new(Account::TYPE, AccountEventPayload::FUNDS_RESERVED),
            EventSelector::new(Account::TYPE, AccountEventPayload::RESERVED_FUNDS_RELEASED),
            EventSelector::new(Account::TYPE, AccountEventPayload::RESERVED_FUNDS_COMMITTED),
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
