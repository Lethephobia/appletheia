use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

/// Projector specification for currency fragments.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CurrencyFragmentProjectorSpec;

impl ProjectorSpec for CurrencyFragmentProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("currency_fragment"),
        Subscription::AnyOf(&[
            EventSelector::new::<Currency>(CurrencyEventPayload::DEFINED),
            EventSelector::new::<Currency>(CurrencyEventPayload::PROVISIONED),
            EventSelector::new::<Currency>(CurrencyEventPayload::OWNERSHIP_TRANSFERRED),
            EventSelector::new::<Currency>(CurrencyEventPayload::SYMBOL_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::NAME_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::DESCRIPTION_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::IMAGE_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::SUPPLY_COMMITTED),
            EventSelector::new::<Currency>(CurrencyEventPayload::ACTIVATED),
            EventSelector::new::<Currency>(CurrencyEventPayload::DEACTIVATED),
            EventSelector::new::<Currency>(CurrencyEventPayload::REMOVED),
        ]),
    );
}
