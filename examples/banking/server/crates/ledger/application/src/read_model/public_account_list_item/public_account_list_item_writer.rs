use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use appletheia::domain::EventOccurredAt;
use banking_ledger_domain::account::{AccountId, AccountOwner};
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};

use super::{PublicAccountListItemStatus, PublicAccountListItemWriterError};

#[allow(async_fn_in_trait, clippy::too_many_arguments)]
pub trait PublicAccountListItemWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_account(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        owner: AccountOwner,
        currency_id: CurrencyId,
        status: PublicAccountListItemStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), PublicAccountListItemWriterError>;

    async fn update_account_owner(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        owner: AccountOwner,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), PublicAccountListItemWriterError>;

    async fn update_account_status(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        status: PublicAccountListItemStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), PublicAccountListItemWriterError>;

    async fn delete_account(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        event_sequence: EventSequence,
    ) -> Result<(), PublicAccountListItemWriterError>;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        name: CurrencyName,
        decimals: CurrencyDecimals,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), PublicAccountListItemWriterError>;

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), PublicAccountListItemWriterError>;

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        name: CurrencyName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), PublicAccountListItemWriterError>;

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        event_sequence: EventSequence,
    ) -> Result<(), PublicAccountListItemWriterError>;
}
