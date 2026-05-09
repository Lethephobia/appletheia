use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

/// Projector specification for public account list item read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PublicAccountListItemProjectorSpec;

impl ProjectorSpec for PublicAccountListItemProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("public_account_list_item"),
        Subscription::AnyOf(&[
            EventSelector::new::<Account>(AccountEventPayload::OPENED),
            EventSelector::new::<Account>(AccountEventPayload::OWNERSHIP_TRANSFERRED),
            EventSelector::new::<Account>(AccountEventPayload::FROZEN),
            EventSelector::new::<Account>(AccountEventPayload::THAWED),
            EventSelector::new::<Account>(AccountEventPayload::CLOSED),
            EventSelector::new::<Currency>(CurrencyEventPayload::DEFINED),
            EventSelector::new::<Currency>(CurrencyEventPayload::SYMBOL_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::NAME_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::REMOVED),
        ]),
    );
}
