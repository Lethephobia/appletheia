use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

/// Declares the subscription for the currency projection projector.
pub struct CurrencyProjectorSpec;

impl ProjectorSpec for CurrencyProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("currency"),
        Subscription::AnyOf(&[
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::DEFINED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::OWNERSHIP_TRANSFERRED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::SYMBOL_CHANGED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::NAME_CHANGED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::SUPPLY_INCREASED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::SUPPLY_DECREASED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::ACTIVATED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::DEACTIVATED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::REMOVED),
        ]),
    );
}
