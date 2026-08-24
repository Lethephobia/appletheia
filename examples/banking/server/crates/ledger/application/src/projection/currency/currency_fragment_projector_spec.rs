use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};
use banking_ledger_domain::token_binding::{TokenBinding, TokenBindingEventPayload};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CurrencyFragmentProjectorSpec;

impl ProjectorSpec for CurrencyFragmentProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("currency_fragment"),
        Subscription::AnyOf(&[
            EventSelector::new::<Currency>(CurrencyEventPayload::DEFINED),
            EventSelector::new::<Currency>(CurrencyEventPayload::DESCRIPTION_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::ACTIVATED),
            EventSelector::new::<Currency>(CurrencyEventPayload::DEACTIVATED),
            EventSelector::new::<TokenBinding>(TokenBindingEventPayload::DEFINED),
            EventSelector::new::<TokenBinding>(TokenBindingEventPayload::REMOVED),
        ]),
    );
}
