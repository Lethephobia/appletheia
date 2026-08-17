use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceEventPayload};
use banking_ledger_domain::deposit::{Deposit, DepositEventPayload};
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload};
use banking_ledger_domain::withdrawal::{Withdrawal, WithdrawalEventPayload};

/// Projector specification for account transaction fragment read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AccountTransactionFragmentProjectorSpec;

impl ProjectorSpec for AccountTransactionFragmentProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("account_transaction_fragment"),
        Subscription::AnyOf(&[
            EventSelector::new::<Deposit>(DepositEventPayload::REQUESTED),
            EventSelector::new::<Deposit>(DepositEventPayload::COMPLETED),
            EventSelector::new::<Deposit>(DepositEventPayload::FAILED),
            EventSelector::new::<Withdrawal>(WithdrawalEventPayload::REQUESTED),
            EventSelector::new::<Withdrawal>(WithdrawalEventPayload::COMPLETED),
            EventSelector::new::<Withdrawal>(WithdrawalEventPayload::FAILED),
            EventSelector::new::<Transfer>(TransferEventPayload::REQUESTED),
            EventSelector::new::<Transfer>(TransferEventPayload::COMPLETED),
            EventSelector::new::<Transfer>(TransferEventPayload::FAILED),
            EventSelector::new::<CurrencyIssuance>(CurrencyIssuanceEventPayload::ISSUED),
            EventSelector::new::<CurrencyIssuance>(CurrencyIssuanceEventPayload::COMPLETED),
            EventSelector::new::<CurrencyIssuance>(CurrencyIssuanceEventPayload::FAILED),
        ]),
    );
}
