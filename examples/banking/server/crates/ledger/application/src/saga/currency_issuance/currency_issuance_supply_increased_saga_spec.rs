use appletheia::application::event::EventSelector;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaPredecessor, SagaSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::CurrencyIssuanceIssuedSagaSpec;

pub struct CurrencyIssuanceSupplyIncreasedSagaSpec;

impl SagaSpec for CurrencyIssuanceSupplyIncreasedSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("currency_issuance_supply_increased"),
        EventSelector::new(Currency::TYPE, CurrencyEventPayload::SUPPLY_INCREASED),
        SagaPredecessor::Required(&CurrencyIssuanceIssuedSagaSpec::DESCRIPTOR),
    );
}
