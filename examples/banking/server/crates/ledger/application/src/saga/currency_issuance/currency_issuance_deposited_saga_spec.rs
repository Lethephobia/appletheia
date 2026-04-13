use appletheia::application::event::EventSelector;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaPredecessor, SagaSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountEventPayload};

use super::CurrencyIssuanceSupplyIncreasedSagaSpec;

pub struct CurrencyIssuanceDepositedSagaSpec;

impl SagaSpec for CurrencyIssuanceDepositedSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("currency_issuance_deposited"),
        EventSelector::new(Account::TYPE, AccountEventPayload::DEPOSITED),
        SagaPredecessor::Required(&CurrencyIssuanceSupplyIncreasedSagaSpec::DESCRIPTOR),
    );
}
