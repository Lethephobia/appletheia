use appletheia::application::event::EventSelector;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaPredecessor, SagaSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceEventPayload};

/// Declares the descriptor and state for the currency issuance saga.
pub struct CurrencyIssuanceIssuedSagaSpec;

impl SagaSpec for CurrencyIssuanceIssuedSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("currency_issuance_issued"),
        EventSelector::new(CurrencyIssuance::TYPE, CurrencyIssuanceEventPayload::ISSUED),
        SagaPredecessor::None,
    );
}
