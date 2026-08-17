use appletheia::application::unit_of_work::UnitOfWork;

use appletheia::application::read_model::MaterializationEventContext;
use banking_ledger_domain::account::{AccountId, AccountName, AccountOwner};
use banking_ledger_domain::core::CurrencyAmount;

use super::{
    AccountFragment, AccountFragmentUpsert, AccountFragmentWriterError, MaterializedAccountStatus,
};

#[allow(async_fn_in_trait)]
pub trait AccountFragmentWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_account(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: AccountFragmentUpsert,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError>;

    async fn update_account_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        owner: AccountOwner,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError>;

    async fn update_account_name(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        name: AccountName,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError>;

    async fn increase_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError>;

    async fn decrease_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError>;

    async fn reserve_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError>;

    async fn release_reserved_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError>;

    async fn commit_reserved_balance(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError>;

    async fn update_account_status(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
        status: MaterializedAccountStatus,
    ) -> Result<Option<AccountFragment>, AccountFragmentWriterError>;

    async fn delete_account(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountId,
    ) -> Result<bool, AccountFragmentWriterError>;
}
