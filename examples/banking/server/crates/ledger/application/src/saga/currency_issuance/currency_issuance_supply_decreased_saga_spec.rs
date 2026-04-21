use appletheia::application::event::EventSelector;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaPredecessor, SagaSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::CurrencyIssuanceSupplyIncreasedSagaSpec;

pub struct CurrencyIssuanceSupplyDecreasedSagaSpec;

impl SagaSpec for CurrencyIssuanceSupplyDecreasedSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("currency_issuance_supply_decreased"),
        EventSelector::new(Currency::TYPE, CurrencyEventPayload::SUPPLY_DECREASED),
        SagaPredecessor::Required(&CurrencyIssuanceSupplyIncreasedSagaSpec::DESCRIPTOR),
    );
}
