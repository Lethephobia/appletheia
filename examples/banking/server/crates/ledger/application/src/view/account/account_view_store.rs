use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::account::{AccountId, AccountName, AccountOwner, AccountStatus};
use banking_ledger_domain::core::CurrencyAmount;

use super::{AccountViewStoreError, AccountViewUpsert};

/// Persists normalized account views for query-side reads.
#[allow(async_fn_in_trait)]
pub trait AccountViewStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: AccountViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError>;

    async fn update_owner(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        owner: AccountOwner,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError>;

    async fn update_name(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        name: AccountName,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError>;

    async fn increase_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError>;

    async fn decrease_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError>;

    async fn move_balance_to_reserved(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError>;

    async fn move_reserved_to_balance(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError>;

    async fn decrease_reserved(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: AccountId,
        status: AccountStatus,
        event_sequence: EventSequence,
    ) -> Result<(), AccountViewStoreError>;
}
