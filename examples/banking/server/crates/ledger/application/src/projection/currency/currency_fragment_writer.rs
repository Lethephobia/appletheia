use appletheia::application::unit_of_work::UnitOfWork;

use appletheia::application::read_model::MaterializationEventContext;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyDescription, CurrencyId, CurrencyImageRef, CurrencyName, CurrencyOwner, CurrencySymbol,
    MintAccountAddress,
};

use super::{
    CurrencyFragment, CurrencyFragmentUpsert, CurrencyFragmentWriterError,
    MaterializedCurrencyStatus,
};

#[allow(async_fn_in_trait)]
pub trait CurrencyFragmentWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: CurrencyFragmentUpsert,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn update_currency_owner(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        owner: CurrencyOwner,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        symbol: CurrencySymbol,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        name: CurrencyName,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn update_currency_description(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        description: Option<CurrencyDescription>,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn update_currency_image(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        image: Option<CurrencyImageRef>,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn provision_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        mint_account_address: MintAccountAddress,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn increase_currency_supply(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        amount: CurrencyAmount,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn update_currency_status(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
        status: MaterializedCurrencyStatus,
    ) -> Result<Option<CurrencyFragment>, CurrencyFragmentWriterError>;

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: CurrencyId,
    ) -> Result<bool, CurrencyFragmentWriterError>;
}
