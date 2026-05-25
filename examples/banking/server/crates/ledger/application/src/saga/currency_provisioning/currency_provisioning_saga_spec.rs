use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::CurrencyProvisioningSagaState;

/// Declares the descriptor and state for the currency provisioning saga.
pub struct CurrencyProvisioningSagaSpec;

impl SagaSpec for CurrencyProvisioningSagaSpec {
    type State = CurrencyProvisioningSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("currency_provisioning"),
        SagaStartEvents::new(&[EventSelector::new::<Currency>(
            CurrencyEventPayload::DEFINED,
        )]),
        Subscription::AnyOf(&[
            EventSelector::new::<Currency>(CurrencyEventPayload::DEFINED),
            EventSelector::new::<Currency>(CurrencyEventPayload::PROVISIONED),
            EventSelector::new::<Currency>(CurrencyEventPayload::PROVISION_REJECTED),
        ]),
    );
}
