use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceEventPayload};
use banking_ledger_domain::deposit::{Deposit, DepositEventPayload};
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload};
use banking_ledger_domain::withdrawal::{Withdrawal, WithdrawalEventPayload};

/// Projector specification for owned account transaction list read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OwnedAccountTransactionListProjectorSpec;

impl ProjectorSpec for OwnedAccountTransactionListProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("owned_account_transaction_list"),
        Subscription::AnyOf(&[
            EventSelector::new::<Deposit>(DepositEventPayload::REQUESTED),
            EventSelector::new::<Deposit>(DepositEventPayload::COMPLETED),
            EventSelector::new::<Deposit>(DepositEventPayload::FAILED),
            EventSelector::new::<Withdrawal>(WithdrawalEventPayload::REQUESTED),
            EventSelector::new::<Withdrawal>(WithdrawalEventPayload::COMPLETED),
            EventSelector::new::<Withdrawal>(WithdrawalEventPayload::FAILED),
            EventSelector::new::<User>(UserEventPayload::REGISTERED),
            EventSelector::new::<User>(UserEventPayload::USERNAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::PICTURE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::CREATED),
            EventSelector::new::<Organization>(OrganizationEventPayload::HANDLE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::PICTURE_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::DEFINED),
            EventSelector::new::<Currency>(CurrencyEventPayload::SYMBOL_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::NAME_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::REMOVED),
            EventSelector::new::<Transfer>(TransferEventPayload::REQUESTED),
            EventSelector::new::<Transfer>(TransferEventPayload::COMPLETED),
            EventSelector::new::<Transfer>(TransferEventPayload::FAILED),
            EventSelector::new::<CurrencyIssuance>(CurrencyIssuanceEventPayload::ISSUED),
            EventSelector::new::<CurrencyIssuance>(CurrencyIssuanceEventPayload::COMPLETED),
            EventSelector::new::<CurrencyIssuance>(CurrencyIssuanceEventPayload::FAILED),
        ]),
    );
}
