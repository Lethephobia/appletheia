use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::{AccountId, AccountName, AccountOwner};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};

use super::{OwnedAccountListItemStatus, OwnedAccountListItemWriterError};

#[allow(async_fn_in_trait, clippy::too_many_arguments)]
pub trait OwnedAccountListItemWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_account(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        owner: AccountOwner,
        name: AccountName,
        currency_id: CurrencyId,
        balance: CurrencyAmount,
        reserved_balance: CurrencyAmount,
        status: OwnedAccountListItemStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListItemWriterError>;

    async fn update_account_owner(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        owner: AccountOwner,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListItemWriterError>;

    async fn update_account_name(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        name: AccountName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListItemWriterError>;

    async fn increase_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListItemWriterError>;

    async fn decrease_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListItemWriterError>;

    async fn reserve_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListItemWriterError>;

    async fn release_reserved_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListItemWriterError>;

    async fn commit_reserved_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListItemWriterError>;

    async fn update_account_status(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        status: OwnedAccountListItemStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListItemWriterError>;

    async fn delete_account(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListItemWriterError>;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        name: CurrencyName,
        decimals: CurrencyDecimals,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListItemWriterError>;

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListItemWriterError>;

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        name: CurrencyName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListItemWriterError>;

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountListItemWriterError>;
}
