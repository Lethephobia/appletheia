use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};
use banking_ledger_domain::currency_issuance::{
    CurrencyIssuance, CurrencyIssuanceEventPayload,
};
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload};

/// Projector specification for owned account transaction list item read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OwnedAccountTransactionListItemProjectorSpec;

impl ProjectorSpec for OwnedAccountTransactionListItemProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("owned_account_transaction_list_item"),
        Subscription::AnyOf(&[
            EventSelector::new(Account::TYPE, AccountEventPayload::DEPOSITED),
            EventSelector::new(Account::TYPE, AccountEventPayload::WITHDRAWN),
            EventSelector::new(User::TYPE, UserEventPayload::REGISTERED),
            EventSelector::new(User::TYPE, UserEventPayload::USERNAME_CHANGED),
            EventSelector::new(User::TYPE, UserEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new(User::TYPE, UserEventPayload::PICTURE_CHANGED),
            EventSelector::new(Organization::TYPE, OrganizationEventPayload::CREATED),
            EventSelector::new(Organization::TYPE, OrganizationEventPayload::HANDLE_CHANGED),
            EventSelector::new(
                Organization::TYPE,
                OrganizationEventPayload::DISPLAY_NAME_CHANGED,
            ),
            EventSelector::new(
                Organization::TYPE,
                OrganizationEventPayload::PICTURE_CHANGED,
            ),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::DEFINED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::SYMBOL_CHANGED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::NAME_CHANGED),
            EventSelector::new(Currency::TYPE, CurrencyEventPayload::REMOVED),
            EventSelector::new(Transfer::TYPE, TransferEventPayload::REQUESTED),
            EventSelector::new(Transfer::TYPE, TransferEventPayload::COMPLETED),
            EventSelector::new(Transfer::TYPE, TransferEventPayload::FAILED),
            EventSelector::new(CurrencyIssuance::TYPE, CurrencyIssuanceEventPayload::ISSUED),
            EventSelector::new(
                CurrencyIssuance::TYPE,
                CurrencyIssuanceEventPayload::COMPLETED,
            ),
            EventSelector::new(CurrencyIssuance::TYPE, CurrencyIssuanceEventPayload::FAILED),
        ]),
    );
}
