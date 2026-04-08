use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency_definition::{
    CurrencyDefinition, CurrencyDefinitionEventPayload,
};
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceEventPayload};

use super::CurrencyIssuanceSagaState;

/// Declares the descriptor and state for the currency issuance saga.
pub struct CurrencyIssuanceSagaSpec;

impl SagaSpec for CurrencyIssuanceSagaSpec {
    type State = CurrencyIssuanceSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("currency_issuance"),
        Subscription::Only(&[
            EventSelector::new(CurrencyIssuance::TYPE, CurrencyIssuanceEventPayload::ISSUED),
            EventSelector::new(
                CurrencyDefinition::TYPE,
                CurrencyDefinitionEventPayload::SUPPLY_INCREASED,
            ),
            EventSelector::new(Account::TYPE, AccountEventPayload::DEPOSITED),
            EventSelector::new(
                CurrencyDefinition::TYPE,
                CurrencyDefinitionEventPayload::SUPPLY_DECREASED,
            ),
            EventSelector::new(
                CurrencyIssuance::TYPE,
                CurrencyIssuanceEventPayload::COMPLETED,
            ),
            EventSelector::new(CurrencyIssuance::TYPE, CurrencyIssuanceEventPayload::FAILED),
        ]),
    );
}
