use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::deposit::{Deposit, DepositEventPayload};

use super::DepositSagaState;

/// Declares the descriptor and state for the deposit saga.
pub struct DepositSagaSpec;

impl SagaSpec for DepositSagaSpec {
    type State = DepositSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("deposit"),
        SagaStartEvents::new(&[EventSelector::new::<Deposit>(
            DepositEventPayload::SETTLEMENT_VERIFIED,
        )]),
        Subscription::AnyOf(&[
            EventSelector::new::<Deposit>(DepositEventPayload::SETTLEMENT_VERIFIED),
            EventSelector::new::<Account>(AccountEventPayload::DEPOSITED),
            EventSelector::new::<Account>(AccountEventPayload::DEPOSIT_REJECTED),
            EventSelector::new::<Deposit>(DepositEventPayload::COMPLETED),
            EventSelector::new::<Deposit>(DepositEventPayload::FAILED),
        ]),
    );
}
